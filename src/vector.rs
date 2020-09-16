fn main() {
  // vector
  // vector 用来储存一系列的值

  // 新建 vector
  // 注意这里我们增加了一个类型注解。
  // 因为没有向这个 vector 中插入任何值，Rust 并不知道我们想要储存什么类型的元素
  let _v: Vec<i32> = Vec::new();
  let mut v = vec![1, 2, 3];

  // 更新 vector
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  // 丢弃 vector 时也会丢弃其所有元素
  // 当 vector 被丢弃时，所有其内容也会被丢弃，这意味着这里它包含的整数将被清理
  {
    let _v = vec![1, 2, 3];
    // 处理变量 v
  } // <- 这里 v 离开作用域并被丢弃

  // 读取 vector 的元素
  // 对于第一个 [] 方法，当引用一个不存在的元素时 Rust 会造成 panic
  // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None
  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  match v.get(10) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  }
  println!("{:?}", v);

  // 遍历 vector 中的元素
  // 如果想要依次访问 vector 中的每一个元素，我们可以遍历其所有的元素而无需通过索引一次一个的访问
  for i in &v {
    println!("{}", i);
  }

  let mut vv = vec![1, 2, 3];
  for i in &mut vv {
    *i += 50;
  }
  println!("{:?}", vv);

  // 使用枚举来储存多种类型
  #[derive(Debug)]
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(1),
    SpreadsheetCell::Float(2.2),
    SpreadsheetCell::Text(String::from("str")),
  ];
  println!("{:?}", row)
}
