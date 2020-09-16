fn main() {
  // Slice类型
  // 另一个没有所有权的数据类型是 slice。
  // slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合
  let s = String::from("hello world");
  let len = first_word(&s);
  println!("{}", len);
  // s.clear();
  // println!("{}", len);

  // 字符串 slice
  // 字符串 slice（string slice）是 String 中一部分值的引用
  // 这类似于引用整个 String 不过带有额外的 [0..5] 部分
  // 它不是对整个 String 的引用，而是对部分 String 的引用
  // let hello = &s[0..5];
  let hello = &s[..5]; // 等价于上面的写法
  // let world = &s[6..s.len()];
  let world = &s[6..]; // 等价于上面的写法
  let s1 = &s[..]; // 完整字符串
  println!("{}", s1);
  println!("{}, {}", first_word(&String::from(hello)), first_word(&String::from(world)));
  
  // 字符串字面值就是 slice
  // let s = "Hello, world!";
  
  // 字符串 slice 作为参数
  println!("{}, {}, {}", first_word2(&s[..]), first_word2(&hello), first_word2(&world));
  println!("{}", first_word2("helloaaaa"));

  // 其他类型的 slice
  let arr = [1, 2, 3, 4, 5];
  let slice = &arr[..3];
  for ele in slice {
    println!("{}", ele);
  }
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  // iter 方法返回集合中的每一个元素，而 enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回
  // enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i]
    }
  }

  &s[..]
}

fn first_word2(s: &str) -> &str {
  let bytes = s.as_bytes();

  // iter 方法返回集合中的每一个元素，而 enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回
  // enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i]
    }
  }

  &s[..]
}
