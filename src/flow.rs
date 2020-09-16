fn main() {
  // if表达式
  let number = 7;
  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  // 使用 else if 处理多重条件
  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }

  // 在 let 语句中使用 if
  let number = if number == 5 {
    number
  } else {
    10
  };
  println!("{}", number);

  // 使用循环重复执行
  // Rust 有三种循环：loop、while 和 for

  // 使用 loop 重复执行代码
  // loop {
  //   println!("again");
  // }

  // 从循环返回
  // loop 的一个用例是重试可能会失败的操作，比如检查线程是否完成了任务。
  // 然而你可能会需要将操作的结果传递给其它的代码。
  // 如果将返回值加入你用来停止循环的 break 表达式，它会被停止的循环返回
  let mut counter = 0;
  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };
  println!("The result is {}", result);

  // while 条件循环
  let mut number = 3;
  while number != 0 {
    println!("{}", number);

    number = number - 1;
  };
  println!("LIFTOFF!!!!");

  // 使用 for 遍历集合
  let arr = [10, 20, 30, 40, 50];
  for ele in arr.iter() {
    println!("val is: {}", ele);
  }

  // (1..4) range
  // rev，用来反转 range
  for num in (1..4).rev() {
    println!("{}!", num);
  }
  println!("LIFTOFF!!!!");
}
