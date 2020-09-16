fn main() {
  // 方法语法
  // 方法 与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，
  // 同时包含在某处调用该方法时会执行的代码
  // 不过方法与函数是不同的，因为它们在结构体的上下文中被定义
  // 并且它们第一个参数总是 self，它代表调用该方法的结构体实例

  // 定义方法
  // 当使用 object.something() 调用方法时，
  // Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配
  let rect1 = Rectangle { width: 30, height: 50 };
  let rect2 = Rectangle { width: 10, height: 20 };
  let rect3 = Rectangle { width: 50, height: 80 };
  println!("{}", rect1.area());

  // 这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— self 的类型
  // 在给出接收者和方法名的前提下，
  // Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）
  // 事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好
  // p1.distance(&p2);
  // (&p1).distance(&p2);
  println!("{}", rect1.can_hold(&rect2));
  println!("{}", rect1.can_hold(&rect3));

  // 关联函数
  // impl 块的另一个有用的功能是：允许在 impl 块中定义 不 以 self 作为参数的函数。
  // 这被称为 关联函数（associated functions），因为它们与结构体相关联
  // 关联函数经常被用作返回一个结构体新实例的构造函数
  println!("{:?}", Rectangle::square(10));
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  
  // 这个方法位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }
}

// 多个 impl 块
impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
