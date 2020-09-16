fn main() {
  // 泛型、trait 和生命周期
  // trait，这是一个定义泛型行为的方法。
  // trait 可以与泛型结合来将泛型限制为拥有特定行为的类型，而不是任意类型

  // 生命周期（lifetimes），它是一类允许我们向编译器提供引用如何相互关联的泛型。
  // Rust 的生命周期功能允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性

  // 提取函数来减少重复
  let number_list = vec![34, 50, 25, 100, 65];
  let res = largest(&number_list);
  println!("The largest number is {}", res);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = largest(&number_list);
  println!("The largest number is {}", result);
}

fn largest(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}
