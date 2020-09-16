fn main() {
  // 引用与借用（reference_borrowing）

  // 这些 & 符号就是 引用，它们允许你使用值但不获取其所有权
  // 我们将获取引用作为函数参数称为 借用（borrowing）
  // 与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("s1 len is: {}", len);

  // 可变引用
  let mut s = String::from("hello");
  change(&mut s);
  println!("{}", s);
  
  // 不过可变引用有一个很大的限制：在特定作用域中的特定数据只能有一个可变引用
  // let _r1 = &mut s;
  // let _r2 = &mut s;

  // 类似的规则也存在于同时使用可变与不可变引用中
  
  // 数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
  // 1. 两个或更多指针同时访问同一数据。
  // 2. 至少有一个指针被用来写入数据。
  // 3. 没有同步数据访问的机制。

  {
    let _r1 = &mut s;
  } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
  let _r2 = &mut s;

  // let r1 = &s; // 没问题
  // let r2 = &s; // 没问题
  // let r3 = &mut s; // 大问题

  // println!("{}, {}, and {}", r1, r2, r3);

  // 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
  let r1 = &s; // 没问题
  let r2 = &s; // 没问题
  println!("{}, {}", r1, r2);

  let r3 = &mut s; // 大问题
  println!("{}", r3);

  // 悬垂引用（Dangling References）
  // 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer）
  // 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者

  println!("{}", dangle());
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(s: &mut String) {
  s.push_str(", world!");
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用
//   let s = String::from("hello"); // s 是一个新字符串

//   &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放

// 这样就没有任何错误了。所有权被移动出去，所以没有值被释放
fn dangle() -> String { // dangle 返回一个字符串的引用
  let s = String::from("hello1"); // s 是一个新字符串

  s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放

// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用
// 引用必须总是有效的
