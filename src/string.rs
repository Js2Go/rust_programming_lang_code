fn main() {
  // 使用字符串存储 UTF-8 编码的文本
  // String 的类型是由标准库提供的，而没有写进核心语言部分，
  // 它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型

  // 新建字符串
  let mut s = String::new();

  // 使用 to_string 方法从字符串字面值创建 String
  // let data = "initial contents";
  // let s = data.to_string();
  // let s = "initial contents".to_string();
  // let s = String::from("initial contents");

  // 更新字符串
  // String 的大小可以增加，其内容也可以改变，就像可以放入更多数据来改变 Vec 的内容一样。
  // 另外，可以方便的使用 + 运算符或 format! 宏来拼接 String 值
  // 使用 push_str 和 push 附加字符串
  // 可以通过 push_str 方法来附加字符串 slice
  // push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中
  let s2 = "bar";
  s.push_str(s2);
  s.push('A');
  println!("{}", s);
  
  // 使用 + 运算符或 format! 宏拼接字符串
  let s1 = String::from("s1");
  let s2 = String::from("s2");
  // 字符串拼接类似于下面这个函数的签名
  // fn add(self, s: &str) -> String {}
  // 之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str
  // 当add函数被调用时，Rust 使用了一个被称为 解引用强制多态（deref coercion）的技术，
  // 你可以将其理解为它把 &s2 变成了 &s2[..]
  let s3 = s1 + &s2; // 第二个参数现在是&String，而不是&str
  println!("{}", s3);

  let s = format!("{}-{}", s2, s2);
  println!("{}", s);

  // 索引字符串
  // 内部表现
  // String 是一个 Vec<u8> 的封装
  // 每个 Unicode 标量值需要两个字节存储

  // 字节、标量值和字形簇！天呐！
  // Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间 (O(1))。
  // 但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符

  // 字符串 slice
  // 索引字符串通常是一个坏点子，因为字符串索引应该返回的类型是不明确的：字节值、字符、字形簇或者字符串 slice
  let hello = "Здравствуйте";
  println!("{}", &hello[..4]);

  // 遍历字符串的方法
  // 如果你需要操作单独的 Unicode 标量值，最好的选择是使用 chars 方法。
  // 对 “नमस्ते” 调用 chars 方法会将其分开并返回六个 char 类型的值，接着就可以遍历其结果来访问每一个元素了：
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  // bytes 方法返回每一个原始字节，这可能会适合你的使用场景
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}
