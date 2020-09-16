fn main() {
  // 整型
  // 8-bit i8 u8
  // 16-bit i16 u16
  // 32-bit i32 u32（默认类型）
  // 64-bit i64 u64
  // 128-bit i128 u128
  // arch isize usize

  // 除 byte 以外的所有数字字面值允许使用类型后缀，例如 57u8，
  // 同时也允许使用 _ 做为分隔符以方便读数，例如1_000
  // Decimal（十进制） 98_222
  // Hex（十六进制） 0xff
  // Octal（八进制） 0o77
  // Binary（二进制） 0b1111_0000
  // Byte（单字节字符）（仅限于u8） b'A'
  let x: u8 = b'A';
  println!("x is {}", x);
  

  // 浮点型
  // f32 单精度浮点数
  // f64 双精度浮点数 （默认类型）

  let sum = 5 + 10;
  let difference = 95.5 - 4.3;
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let remainder = 43 % 5;
  println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);


  // 布尔值
  // bool true false
  let t = true;
  let f: bool = false;
  println!("{}, {}", t, f);


  // 字符类型

  // Rust 的 char 类型的大小为四个字节(four bytes)，
  // 并代表了一个 Unicode 标量值（Unicode Scalar Value），
  // 这意味着它可以比 ASCII 表示更多内容。
  // 在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。
  // Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。不过，“字符” 并不是一个 Unicode 中的概念，
  // 所以人直觉上的 “字符” 可能与 Rust 中的 char 并不符合
  let c = 'z';
  let z = 'Z';
  let heart_eyed_cat = '😻';
  println!("{} {} {}", c, z, heart_eyed_cat);



  // 复合类型
  // 复合类型（Compound types）可以将多个值组合成一个类型。
  // Rust 有两个原生的复合类型：元组（tuple）和数组（array）

  // 元组类型
  // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
  // 元组长度固定：一旦声明，其长度不会增大或缩小

  // 我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。
  // 元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  let f = tup.0;
  let s = tup.1;
  let t = tup.2;
  println!("{} {} {}", x, y, z);
  println!("{} {} {}", f, s, t);


  // 数组类型
  // 数组中的每个元素的类型必须相同。
  // Rust 中的数组与一些其他语言中的数组不同，
  // 因为 Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小

  // 数组是一整块分配在栈上的内存。可以使用索引来访问数组的元素
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  let arr2 = [1; 5];
  println!("{} {}", arr[3], arr2[4])
}
