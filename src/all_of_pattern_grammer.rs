fn main() {
  // 所有的模式语法
  // 匹配字面值
  let x = 1;
  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("any"),
  }

  
  // 匹配命名变量
  // 命名变量是匹配任何值的不可反驳模式
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
  }
  println!("at the end: x = {:?}, y = {:?}", x, y);

  
  // 多个模式
  let x = 1;
  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }


  // 通过 ..= 匹配值的范围
  // ..= 语法允许你匹配一个闭区间范围内的值
  // 范围只允许用于数字或 char 值
  let x = 5;
  match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
  }

  let x = 'c';
  match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
  }


  // 解构并分解值
  // 解构结构体
  struct Point {
    x: i32,
    y: i32,
  }
  let p = Point { x: 0, y: 7 };
  let Point { x: a, y: b } = p;
  assert_eq!(0, a);
  assert_eq!(7, b);
  let Point { x, y } = p;
  assert_eq!(0, x);
  assert_eq!(7, y);

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }


  // 解构枚举
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }
  let msg = Message::ChangeColor(0, 160, 255);
  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.");
    }
    Message::Move {x, y} => {
      println!(
        "Move in the x direction {} and in the y direction {}",
        x,
        y
      );
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!(
        "Change the color to red {}, green {}, and blue {}",
        r,
        g,
        b
      );
    }
  }


  // 解构嵌套的结构体和枚举
  enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
  }
  enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
  }
  let msg = Message1::ChangeColor(Color::Hsv(0, 160, 255));
  match msg {
    Message1::ChangeColor(Color::Rgb(r, g, b)) =>  {
      println!(
        "Change the color to red {}, green {}, and blue {}",
        r,
        g,
        b
      );
    }
    Message1::ChangeColor(Color::Hsv(h, s, v)) => {
      println!(
        "Change the color to hue {}, saturation {}, and value {}",
        h,
        s,
        v
      );
    }
    _ => ()
  }


  // 解构结构体和元组
  let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });


  // 忽略模式中的值
  // 使用 _ 忽略整个值
  foo(3, 4);


  // 使用嵌套的 _ 忽略部分值
  let mut setting_value = Some(5);
  let new_setting_value = Some(10);
  match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
      println!("Can't overwrite an existing customized value");
    }
    _ => {
      setting_value = new_setting_value;
    }
  }
  println!("setting is {:?}", setting_value);

  let numbers = (2, 4, 8, 16, 32);
  match numbers {
    (first, _, third, _, fifth) => {
      println!("Some numbers: {}, {}, {}", first, third, fifth);
    }
  }


  // 通过在名字前以一个下划线开头来忽略未使用的变量
  let _x = 5;
  let y = 10;

  let s = Some(String::from("Hello"));
  // if let Some(_s) = s {
  if let Some(_) = s {
    println!("found a string");
  }
  println!("{:?}", s);


  // 用 .. 忽略剩余值
  struct Point1 {
    x: i32,
    y: i32,
    z: i32,
  }
  let origin = Point1 { x: 0, y: 0, z: 0 };
  match origin {
    Point1 { x, .. } => println!("x is {}", x),
  }

  let numbers = (2, 4, 8, 16, 32);
  match numbers {
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    }
  }


  // 匹配守卫提供的额外条件
  // 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。
  // 匹配守卫用于表达比单独的模式所能允许的更为复杂的情况
  let num = Some(4);
  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }

  let x = Some(5);
  let y = 10;
  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {}", n),
    _ => println!("Default case, x = {:?}", x),
  }
  println!("at the end: x = {:?}, y = {}", x, y);

  let x = 4;
  let y = false;
  match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }


  // @ 绑定
  // at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
  enum Message2 {
    Hello { id: i32 },
  }
  let msg = Message2::Hello{ id: 5 };
  match msg {
    Message2::Hello { id: id_var @ 3..=7 } => {
      println!("Found an id in range: {}", id_var);
    }
    Message2::Hello { id: 10..=12 } => {
      println!("Found an id in another range")
    }
    Message2::Hello { id } => {
      println!("Found some other id: {}", id)
    }
  }
}

fn foo(_: i32, y: i32) {
  println!("This code only uses the y parameter: {}", y);
}
