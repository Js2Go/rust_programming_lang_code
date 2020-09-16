fn main() {
  // immutable
  // let x = 5;

  // mutable
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
  
  // constant
  // Rust 常量的命名规范是使用下划线分隔的大写字母单词，
  // 并且可以在数字字面值中插入下划线来提升可读性
  const MAX_POINTS: i32 = 1_0000;
  println!("The value of MAX_POINTS is: {}", MAX_POINTS);
  
  // shadow
  let x = x * x;
  let x = x + 1;
  println!("The value of x is: {}", x);
}
