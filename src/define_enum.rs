fn main() {
  // 定义枚举
  // 枚举值
  // 注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开
  let _four = IpAddrKind::V4;
  let _six = IpAddrKind::V6;

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };
  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };

  println!("home: {:#?}, \nloopback: {:#?}", home, loopback);

  // 我们可以使用一种更简洁的方式来表达相同的概念，
  // 仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分
  // 用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据
  let home = IpAddr2::V4(String::from("127.0.0.1"));
  let loopback = IpAddr2::V6(String::from("::1"));
  println!("home: {:#?}, \nloopback: {:#?}", home, loopback);
  
  let home = IpAddr3::V4(127, 0, 0, 1);
  println!("home: {:#?}", home);

  Message::Move{ x: 1, y: 2 }.call();

  // Option 枚举和其相对于空值的优势
  let some_number = Some(5);
  let some_string = Some("a string ");
  let absent_number: Option<i32> = None;
}

#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6,
}

#[derive(Debug)]
enum IpAddr2 {
  V4(String),
  V6(String),
}

#[derive(Debug)]
enum IpAddr3 {
  V4(u8, u8, u8, u8),
  V6(String),
}

#[derive(Debug)]
struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    println!("{:?}", self);
  }
}

// fn route(ip_type: IpAddrKind) {}
