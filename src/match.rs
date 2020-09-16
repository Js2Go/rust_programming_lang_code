fn main() {
  // match 控制流运算符
  println!("{}", value_in_cents(Coin::Penny));

  // 绑定值的模式
  println!("{}", value_in_cents2(Coin2::Quarter(UsState::Alaska)));

  // 匹配 Option<T>
  // 匹配 Some(T)
  println!("{:?}", plus_one(Some(5)));
  println!("{:?}", plus_one(None));

  // 匹配是穷尽的
  // Rust 知道我们没有覆盖所有可能的情况甚至知道哪些模式被忘记了！
  // Rust 中的匹配是 穷尽的（exhaustive）：必须穷举到最后的可能性来使代码有效
  
  // 特别的在这个 Option<T> 的例子中，Rust 防止我们忘记明确的处理 None 的情况，
  // 这使我们免于假设拥有一个实际上为空的值
}

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

enum Coin2 {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn value_in_cents2(coin: Coin2) -> u8 {
  // _ 通配符
  // _ 模式会匹配所有的值。通过将其放置于其他分支之后，
  // _ 将会匹配所有之前没有指定的可能的值。() 就是 unit 值，所以 _ 的情况什么也不会发生
  match coin {
    Coin2::Quarter(state) => {
      println!("{:?}", state);
      25
    },
    _ => 0,
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}
