fn main() {
  // 定义并实例化结构体
  
  // 注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变
  let mut user1 = User {
    username: String::from("mazi"),
    email: String::from("mazi@qq.com"),
    sign_in_count: 1,
    active: true,
  };
  user1.username = String::from("kaikai");
  
  // 同其他任何表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例
  let user = build_user(user1.username, user1.email);
  println!("{}, {}, {}, {}", user.username, user.email, user.sign_in_count, user.active);

  // 使用结构体更新语法从其他实例创建实例
  // let user2 = User {
  //   email: String::from("another@example.com"),
  //   username: String::from("anotherusername567"),
  //   sign_in_count: user1.sign_in_count,
  //   active: user1.active,
  // };
  let _user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };

  // 使用没有命名字段的元组结构体来创建不同的类型
  // 也可以定义与元组（在第三章讨论过）类似的结构体，称为 元组结构体（tuple structs）
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  let black: Color = Color(0, 0, 0);
  let origin: Point = Point(1, 0, 0);
  println!("{} {} {}", black.0, black.1, black.2);
  println!("{} {} {}", origin.0, origin.1, origin.2);

  // 没有任何字段的类单元结构体
  // 我们也可以定义一个没有任何字段的结构体！它们被称为 类单元结构体（unit-like structs）因为它们类似于 ()，
  // 即 unit 类型。类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用

  // 结构体数据的所有权
  // struct User2 {
  //   username: &str,
  //   email: &str,
  //   sign_in_count: u64,
  //   active: bool,
  // }

  // let user2 = User2 {
  //   email: "someone@example.com",
  //   username: "someusername123",
  //   active: true,
  //   sign_in_count: 1,
  // };
}

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn build_user(username: String, email: String) -> User {
  // User {
  //   username: username,
  //   email: email,
  //   active: true,
  //   sign_in_count: 1,
  // }
  // 变量与字段同名时的字段初始化简写语法
  User {
    username,
    email,
    active: true,
    sign_in_count: 1,
  }
}
