fn main() {
  // 模式用来匹配值的结构
  // 所有可能会用到模式的位置
  // match 分支
  // match 表达式必须是 穷尽（exhaustive）的，意为 match 表达式所有可能的值都必须被考虑到
  // 有一个特定的模式 _ 可以匹配所有情况，不过它从不绑定任何变量

  // if let 条件表达式
  let favorite_color: Option<&str> = Some("green");
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background color");
    } else {
      println!("Using orange as the background color");
    }
  } else {
    println!("Using blue as the background color");
  }

  // while let 条件循环
  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  while let Some(top) = stack.pop() {
    println!("{}", top);
  }

  // for 循环
  let v = vec!['a', 'b', 'c'];
  for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
  }

  // let 语句
  let (x, y, z) = (1, 2, 'a');
  println!("{} {} {}", x, y, z);

  print_coordinates(&(3, 5));
}

// 函数参数
fn print_coordinates(&(x, y): &(i32, i32)) {
  println!("Current location: ({}, {})", x, y);
}
