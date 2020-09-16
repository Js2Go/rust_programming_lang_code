fn main() {
  // 使用迭代器处理元素序列
  // 迭代器模式允许你对一个项的序列进行某些处理。
  // 迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。
  // 当使用迭代器时，我们无需重新实现这些逻辑

  // 在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  for val in v1_iter {
    println!("Got: {}", val);
  }

  // Iterator trait 和 next 方法

  // 消费迭代器的方法

  // 产生其他迭代器的方法

  // 使用闭包获取环境

  // 实现 Iterator trait 来创建自定义迭代器
}

#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter()
    .filter(|s| s.size == shoe_size)
    .collect()
}

#[test]
fn filters_by_size() {
  let shoes = vec![
    Shoe { size: 10, style: String::from("sneaker") },
    Shoe { size: 13, style: String::from("sandal") },
    Shoe { size: 10, style: String::from("boot") },
  ];

  let in_my_size = shoes_in_my_size(shoes, 10);

  assert_eq!(
    in_my_size,
    vec![
      Shoe { size: 10, style: String::from("sneaker") },
      Shoe { size: 10, style: String::from("boot") },
    ]
  )
}

#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];

  let mut v1_iter = v1.iter();
  
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
  
  let v1_iter = v1.iter();
  let total: i32 = v1_iter.sum();
  assert_eq!(total, 6);

  let v1: Vec<i32> = vec![1, 2, 3];
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  assert_eq!(v2, vec![2, 3, 4]);
}

struct Counter {
  count: u32,
}
impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}
impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;

    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

// 使用 Counter 迭代器的 next 方法
#[test]
fn calling_next_directly() {
  let mut counter = Counter::new();

  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), Some(5));
  assert_eq!(counter.next(), None);
}

// 使用自定义迭代器中其他 Iterator trait 方法
#[test]
fn using_other_iterator_trait_methods() {
  let sum: u32 = Counter::new().zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();

  assert_eq!(18, sum);
}
