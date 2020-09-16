use std::thread;
use std::time::Duration;

fn main() {
  // 闭包：可以捕获环境的匿名函数
  // Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数

  // 使用闭包创建行为的抽象
  let simulated_use_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(
    simulated_use_specified_value,
    simulated_random_number
  );
}

struct Cacher<T>
  where T: Fn(u32) -> u32
{
  calculation: T,
  value: Option<u32>,
}
impl<T> Cacher<T>
  where T: Fn(u32) -> u32
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      },
    }
  }
}

#[test]
fn call_with_different_values() {
  let mut c = Cacher::new(|a| a);

  let v1 = c.value(1);
  let v2 = c.value(2);

  assert_eq!(v2, 2);
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_result = Cacher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });
  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      expensive_result.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_result.value(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_result.value(intensity)
      );
    }
  }

  // 重构使用闭包储存代码
  // let expensive_closure = |num| {
  //   println!("calculating slowly...");
  //   thread::sleep(Duration::from_secs(2));
  //   num
  // };
  // if intensity < 25 {
  //   println!(
  //     "Today, do {} pushups!",
  //     expensive_closure(intensity)
  //   );
  //   println!(
  //     "Next, do {} situps!",
  //     expensive_closure(intensity)
  //   );
  // } else {
  //   if random_number == 3 {
  //     println!("Take a break today! Remember to stay hydrated");
  //   } else {
  //     println!(
  //       "Today, run for {} minutes!",
  //       expensive_closure(intensity)
  //     );
  //   }
  // }

  // 使用函数重构
  // let expensive_result = simulated_expensive_calculation(intensity);
  // if intensity < 25 {
  //   println!(
  //     "Today, do {} pushups!",
  //     expensive_result
  //   );
  //   println!(
  //     "Next, do {} situps!",
  //     expensive_result
  //   );
  // } else {
  //   if random_number == 3 {
  //     println!("Take a break today! Remember to stay hydrated");
  //   } else {
  //     println!(
  //       "Today, run for {} minutes!",
  //       expensive_result
  //     );
  //   }
  // }
  // if intensity < 25 {
  //   println!(
  //     "Today, do {} pushups!",
  //     simulated_expensive_calculation(intensity)
  //   );
  //   println!(
  //     "Next, do {} situps!",
  //     simulated_expensive_calculation(intensity)
  //   );
  // } else {
  //   if random_number == 3 {
  //     println!("Take a break today! Remember to stay hydrated");
  //   } else {
  //     println!(
  //       "Today, run for {} minutes!",
  //       simulated_expensive_calculation(intensity)
  //     );
  //   }
  // }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
  println!("calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  intensity
}
