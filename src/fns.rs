fn main() {
  // 函数
  // Rust 代码中的函数和变量名使用 snake case 规范风格。
  // 在 snake case 中，所有字母都是小写并使用下划线分隔单词

  // 在函数签名中，必须 声明每个参数的类型
  another_fn(5, 10);

  // 包含语句和表达式的函数体
  let y = {
    let x = 3;
    x + 1
  };
  println!("y: {}", y);

  // 具有返回值的函数
  // 我们并不对返回值命名，但要在箭头（->）后声明它的类型。
  // 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值
  // 使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式

  // 使用空元组 () 表示不返回值
  println!("{}", five());
  println!("{}", plus_one(5));
}

fn another_fn(x: i32, y: i32) {
  println!("Another Fn, {}, {}", x, y);
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
