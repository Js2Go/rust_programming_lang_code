fn main() {
  // assert_eq! 和 assert_ne! 宏在底层分别使用了 == 和 !=。
  // 当断言失败时，这些宏会使用调试格式打印出其参数，这意味着被比较的值必需实现了 PartialEq 和 Debug trait

  // 使用 should_panic 检查 panic

  // 并行或连续的运行测试
  // cargo test -- --test-threads=1

  // 显示函数输出
  // cargo test -- --nocapture

  // 通过指定名字来运行部分测试
  // 运行单个测试
  // cargo test larger_can_hold_smaller

  // 过滤运行多个测试
  // 我们可以指定部分测试的名称，任何名称匹配这个名称的测试会被运行
  // cargo test it_works

  // 忽略某些测试
  // #[ignore]
  // 如果我们只希望运行被忽略的测试，可以使用 cargo test -- --ignored

  // tests目录下的文件
  // cargo test --test integration_test
}

struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  fn can_holad(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

#[test]
fn larger_can_hold_smaller() {
  let larger = Rectangle{ width: 8, height: 7 };
  let smaller = Rectangle{ width: 5, height: 1 };

  assert!(larger.can_holad(&smaller));
}

#[test]
#[ignore]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
fn another() {
  // panic!("Make this test fail!");
  assert_ne!(2 + 2, 5);
}

fn greeting(name: &str) -> String {
  format!("Hello {}!", name)
  // String::from("Hello")
}

#[test]
fn greeting_contains_name() {
  let result = greeting("Carol");
  assert!(
    result.contains("Carol"),
    "Greeting did not contain name, value was `{}`",
    result
  );
}

struct Guess {
  value: i32,
}

impl Guess {
  fn new(value: i32) -> Guess {
    // if value < 1 || value > 100 {
    //   panic!("Guess value must be between 1 and 100, got {}.", value);
    // }
    if value < 1 {
      panic!("Guess value must be greater than or equal to 1, got {}.", value);
    } else if value > 100 {
      panic!("Guess value must be less than or equal to 100, got {}.", value);
    }

    Guess {
      value
    }
  }
}

#[test]
// #[should_panic]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
  Guess::new(200);
}

// 将 Result<T, E> 用于测试
#[test]
fn it_works2() -> Result<(), String> {
  if 2 + 2 == 4 {
    Ok(())
  } else {
    Err(String::from("two plus two does not equal four"))
  }
}
