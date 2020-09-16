use std::ops::Deref;

fn main() {
  // 通过 Deref trait 将智能指针当作常规引用处理
  // 通过解引用运算符追踪指针的值
  let x = 5;
  let y = &x;
  assert_eq!(5, x);
  assert_eq!(5, *y);
  
  // 像引用一样使用 Box<T>
  let y = Box::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y);
  
  let y = MyBox::new(x);
  assert_eq!(5, x);
  // *(y.deref())
  assert_eq!(5, *y);

  // 函数和方法的隐式解引用强制多态
  // 解引用强制多态（deref coercions）是 Rust 在函数或方法传参上的一种便利
  // 其将实现了 Deref 的类型的引用转换为原始类型通过 Deref 所能够转换的类型的引用
  // 当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，解引用强制多态将自动发生
  // 这时会有一系列的 deref 方法被调用，把我们提供的类型转换成了参数所需的类型
  let m = MyBox::new(String::from("Rust"));
  hello(&m);
  hello(&(*m)[..]);

  // 解引用强制多态如何与可变性交互
  // 类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符

  // Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：
  // - 当 T: Deref<Target=U> 时从 &T 到 &U。
  // - 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
  // - 当 T: Deref<Target=U> 时从 &mut T 到 &U。
}
// 自定义智能指针
struct MyBox<T>(T);
impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}
// 通过实现 Deref trait 将某类型像引用一样处理
impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}
