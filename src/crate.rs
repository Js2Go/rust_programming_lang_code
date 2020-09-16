fn main() {
  // if let 简单控制流
  // if let 语法让我们以一种不那么冗长的方式结合 if 和 let，
  // 来处理只匹配一个模式的值而忽略其他模式的情况
  // 我们想要对 Some(3) 匹配进行操作但是不想处理任何其他 Some<u8> 值或 None 值。
  // 为了满足 match 表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上 _ => ()，这样也要增加很多样板代码
  let some_u8_value = Some(3u8);
  // match some_u8_value {
  //   Some(3) => println!("three"),
  //   _ => (),
  // }

  if let Some(3) = some_u8_value {
    println!("three");
  }

  // 可以在 if let 中包含一个 else。
  // else 块中的代码与 match 表达式中的 _ 分支块中的代码相同，这样的 match 表达式就等同于 if let 和 else
  let mut count = 0;
  // let coin = Coin::Hhh(String::from("emmm"));
  let coin = Coin::Quarter(UsState::Mazi);
  // match coin {
  //   Coin::Quarter(state) => {
  //     println!("{:?}", state);
  //   },
  //   _ => count += 1,
  // }
  // println!("{}", count);
  if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
  } else {
    count += 1;
  }
  println!("{}", count);
}

#[derive(Debug)]
enum UsState {
  Mazi,
  Kaikai,
}

enum Coin {
  Quarter(UsState),
  Hhh(String),
}
