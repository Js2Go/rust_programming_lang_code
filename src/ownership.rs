fn main() {
  // 所有权（ownership）

  // 所有权规则
  // 1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
  // 2. 值在任一时刻有且只有一个所有者。
  // 3. 当所有者（变量）离开作用域，这个值将被丢弃。
  let _s = "hello"; // 字符串字面量存储在栈上，且不可重新赋值
  // s = "123123";

  // String 类型
  // 这个类型被分配到堆上，所以能够存储在编译时未知大小的文本。
  // 可以使用 from 函数基于字符串字面值来创建 String
  let mut s = String::from("hello");
  s.push_str(" ,world!");
  println!("{}", s);

  // 内存与分配

  // 只在栈上的数据：拷贝
  // 因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中
  let x = 5;
  let _y = x;
  
  // 变量与数据交互的方式（一）：移动
  // 移动 move
  let s1 = String::from("hello");
  let s2 = s1;
  println!("s1 value is: {}", s2);
  // Rust 永远也不会自动创建数据的 “深拷贝”。
  // 因此，任何 自动 的复制可以被认为对运行时性能影响较小

  // 变量与数据交互的方式（二）：克隆
  // 这里堆上的数据 确实 被复制了
  let s3 = s2.clone();
  println!("s2: {}, s3: {}", s2, s3);

  // 所有权与函数
  let s = String::from("hello"); // s 进入作用域
  take_ownership(s); // s 的值移动到函数里 所以到这里不再有效

  let x = 5; // x 进入作用域
  makes_copy(x); // x 应该移动函数里，但 i32 是 Copy 的，所以在后面可继续使用 x

  // 返回值与作用域
  let s1 = give_someownership(); // gives_ownership 将返回值移给 s1
  let s2 = String::from("hello"); // s2 进入作用域
  let s3 = takes_and_gives_back(s2); // s2 被移动到takes_and_gives_back 中,它也将返回值移给 s3
  println!("{} {}", s1, s3); // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生。s1 移出作用域并被丢弃

  let s1 = String::from("hello");
  let (s2, len) = calculate_length(s1);
  println!("The length of '{}' is {}.", s2, len)
}

fn take_ownership(some_str: String) { // some_string 进入作用域
  println!("{}", some_str);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
  println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn give_someownership() -> String { // gives_ownership 将返回值移动给调用它的函数
  let some_str = String::from("hello"); // some_string 进入作用域.
  some_str // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
  a_string // 返回 a_string 并移出给调用的函数
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() 返回字符串的长度

  (s, length)
}
