use std::slice;

fn main() {
  // 不安全 Rust

  // 不安全的超级力量
  // - 解引用裸指针
  // - 调用不安全的函数或方法
  // - 访问或修改可变静态变量
  // - 实现不安全 trait
  // - 访问 union 的字段
  // unsafe 并不会关闭借用检查器或禁用任何其他 Rust 安全检查：如果在不安全代码中使用引用，它仍会被检查

  
  // 解引用裸指针
  // 不安全 Rust 有两个被称为 裸指针（raw pointers）的类似于引用的新类型
  // 和引用一样，裸指针是不可变或可变的，分别写作 *const T 和 *mut T
  // 这里的星号不是解引用运算符；它是类型名称的一部分
  // 在裸指针的上下文中，不可变 意味着指针解引用之后不能直接赋值

  // 与引用和智能指针的区别在于，记住裸指针
  // - 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
  // - 不保证指向有效的内存
  // - 允许为空
  // - 不能实现任何自动清理功能
  let mut num = 5;
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;
  // 注意这里没有引入 unsafe 关键字。
  // 可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针，稍后便会看到 
  // let address = 0x012345usize;
  // let r = address as *const i32;
  // 可以在安全代码中创建裸指针，不过不能 解引用 裸指针和读取其指向的数据
  unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
    // println!("r is: {}", *r);
  }

  
  // 调用不安全函数或方法
  unsafe fn dangerous() {}
  unsafe {
    dangerous();
  }


  // 创建不安全代码的安全抽象
  let mut v = vec![1, 2, 3, 4, 5, 6];
  let r = &mut v[..];
  // let (a, b) = r.split_at_mut(3);
  let (a, b) = split_at_mut(r, 3);

  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);

  fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
      (slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
  }


  // 使用 extern 函数调用外部代码
  // 有时你的 Rust 代码可能需要与其他语言编写的代码交互
  // 为此 Rust 有一个关键字，extern，有助于创建和使用 外部函数接口（Foreign Function Interface， FFI）
  // 外部函数接口是一个编程语言用以定义函数的方式，其允许不同（外部）编程语言调用这些函数
  extern "C" {
    fn abs(input: i32) -> i32;
  }
  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }
  // 从其它语言调用 Rust 函数
  // extern 的使用无需 unsafe
  #[no_mangle]
  pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
  }


  // 访问或修改可变静态变量
  // 目前为止全书都尽量避免讨论 全局变量（global variables），Rust 确实支持他们，不过这对于 Rust 的所有权规则来说是有问题的
  // 如果有两个线程访问相同的可变全局变量，则可能会造成数据竞争
  // 全局变量在 Rust 中被称为 静态（static）变量
  static HELLO_WORLD: &str = "Hello, world!";
  println!("name is: {}", HELLO_WORLD);

  static mut COUNTER: u32 = 0;
  fn add_to_count(inc: u32) {
    unsafe {
      COUNTER += inc;
    }
  }
  add_to_count(3);
  unsafe {
    println!("COUNTER: {}", COUNTER);
  }

  
  // 实现不安全 trait
  unsafe trait Foo {
    // methods go here
  }
  unsafe impl Foo for i32 {
    // method implementations go here
  }


  // 访问联合体中的字段
  // union 和 struct 类似，但是在一个实例中同时只能使用一个声明的字段
  // 联合体主要用于和 C 代码中的联合体交互。访问联合体的字段是不安全的，因为 Rust 无法保证当前存储在联合体实例中数据的类型


  // 何时使用不安全代码
  // 使用 unsafe 来进行这五个操作（超级力量）之一是没有问题的，
  // 甚至是不需要深思熟虑的，不过使得 unsafe 代码正确也实属不易，因为编译器不能帮助保证内存安全
  // 当有理由使用 unsafe 代码时，是可以这么做的，通过使用显式的 unsafe 标注使得在出现错误时易于追踪问题的源头
}
