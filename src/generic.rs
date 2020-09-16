fn main() {
  // 泛型数据类型
  // generic
  
  // 在函数定义中使用泛型
  // 当使用泛型定义函数时，我们在函数签名中通常为参数和返回值指定数据类型的位置放置泛型
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest_i32(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest_char(&char_list);
  println!("The largest char is {}", result);
  
  // fn largest<T>(list: &[T]) -> T
  // 函数 largest 有泛型类型 T。它有一个参数 list，它的类型是一个 T 值的 slice。
  // largest 函数将会返回一个与 T 相同类型的值
  // println!("The largest is {}", largest(&number_list));
  // println!("The largest is {}", largest(&char_list));

  // 结构体定义中的泛型
  #[derive(Debug)]
  struct Point<T, U> {
    x: T,
    y: U,
  }

  let integer = Point{
    x: 5,
    y: 10.0,
  };
  let float = Point{
    x: 1.0,
    y: 4,
  };
  println!("integer: {:?}\nfloat: {:?}", integer, float);

  // 枚举定义中的泛型
  // enum Option<T> {
  //   Some(T),
  //   None,
  // }
  // enum Result<T, E> {
  //   Ok(T),
  //   Err(E),
  // }

  // 方法定义中的泛型
  struct Point2<T> {
    x: T,
    y: T,
  }
  impl<T> Point2<T> {
    fn x(&self) -> &T {
      &self.x
    }
  }
  impl Point2<f32> {
    fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
  }
  let p = Point2{x: 1, y: 2};
  println!("{}", p.x());
  let fp = Point2{x: 3.0, y: 4.0};
  println!("{}", fp.distance_from_origin());
  
  impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
      Point {
        x: self.x,
        y: other.y,
      }
    }
  }
  let p1 = Point{x: 1, y: 2.0};
  let p2 = Point{x: 1, y: 'c'};
  println!("{:?}", p1.mixup(p2));

  // 泛型代码的性能
  // Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
  // 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程
  // 编译器寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码

  // 我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。
  // 这意味着在使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。
  // 这个单态化过程正是 Rust 泛型在运行时极其高效的原因

  // 编译器生成的单态化版本的代码看起来像这样，
  // 并包含将泛型 Option<T> 替换为编译器创建的具体定义后的用例代码
  // enum Option_i32 {
  //   Some(i32),
  //   None,
  // }

  // enum Option_f64 {
  //   Some(f64),
  //   None,
  // }

  // fn main() {
  //   let integer = Option_i32::Some(5);
  //   let float = Option_f64::Some(5.0);
  // }
}

// fn largest<T>(list: &[T]) -> T {
//   let mut largest = list[0];

//   for &item in list.iter() {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }

fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}
