use std::collections::HashMap;

fn main() {
  // 哈希 map 储存键值对
  // 新建一个哈希 map
  // 哈希 map 将它们的数据储存在堆上
  // 类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型
  let mut scores: HashMap<String, i32> = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  // 另一个构建哈希 map 的方法是使用一个元组的 vector 的 collect 方法，其中每个元组包含一个键值对
  // collect 方法可以将数据收集进一系列的集合类型，包括 HashMap
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];
  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

  // 哈希 map 和所有权
  // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
  // 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");
  let mut map = HashMap::new();
  // 当 insert 调用将 field_name 和 field_value 移动到哈希 map 中后，将不能使用这两个绑定
  map.insert(field_name, field_value);

  // 访问哈希 map 中的值
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  // 更新哈希 map
  // 覆盖一个值
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);
  println!("{:?}", scores);
  
  // 只在键没有对应值时插入
  // 为此哈希 map 有一个特有的 API，叫做 entry，它获取我们想要检查的键作为参数
  // entry 函数的返回值是一个枚举，Entry，它代表了可能存在也可能不存在的值
  // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);
  println!("{:?}", scores);

  // 根据旧值更新一个值
  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("{:?}", map);

  // 哈希函数
}
