use std::fs;
use std::fs::File;
// use std::io::ErrorKind;
use std::io;
// use std::io::Read;
use std::error::Error;

// Box<dyn Error> 被称为 “trait 对象”（“trait object”）
fn main() -> Result<(), Box<dyn Error>> {
  // panic! 与不可恢复的错误
  // 对应 panic 时的栈展开或终止
  // 当出现 panic 时，程序默认会开始 展开（unwinding），
  // 这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作
  // 另一种选择是直接 终止（abort），这会不清理数据就退出程序

  // 如果你想要在release模式中 panic 时直接终止（Cargo.toml）
  // [profile.release]
  // panic='abort'
  // panic!("crash and burn");

  // 使用 panic! 的 backtrace
  // let v = vec![1, 2, 3];
  // v[99]; // 缓冲区溢出

  // Result 与可恢复的错误
  // enum Result<T, E> {
  //   Ok(T),
  //   Err(E),
  // }

  // let f = File::open("hello.txt");
  // let _f = match f {
  //   Ok(file) => file,
  //   Err(err) => match err.kind() {
  //     ErrorKind::NotFound => match File::create("hello.txt") {
  //       Ok(fc) => fc,
  //       Err(e) => panic!("Problem creating the file: {:?}", e),
  //     },
  //     other_err => panic!("Problem opening the file: {:?}", other_err),
  //   },
  // };
  // let _f = File::open("hello.txt").unwrap_or_else(|error| {
  //   if error.kind() == ErrorKind::NotFound {
  //     File::create("hello.txt").unwrap_or_else(|error| {
  //       panic!("Problem creating the file: {:?}", error);
  //     })
  //   } else {
  //     panic!("Problem opening the file: {:?}", error);
  //   }
  // });

  // 失败时 panic 的简写：unwrap 和 expect
  // expect 与 unwrap 的使用方式一样：返回文件句柄或调用 panic! 宏
  // let _f = File::open("hello.txt").unwrap();
  // let _f = File::open("hello.txt").expect("Failed to open hello.txt");

  // 传播错误
  // 传播错误的简写：? 运算符
  // 当编写一个其实现会调用一些可能会失败的操作的函数时，
  // 除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。
  // 这被称为 传播（propagating）错误
  let d = read_username_from_file();
  let _d = match d {
    Ok(name) => println!("{}", name),
    Err(e) => panic!("{:?}", e),
  };

  // ? 运算符可被用于返回 Result 的函数
  let f = File::open("hello.txt")?;
  Ok(())
}

// fn read_username_from_file() -> Result<String, io::Error> {
//   let f = File::open("hello.txt");

//   let mut f = match f {
//     Ok(file) => file,
//     Err(error) => return Err(error),
//   };

//   let mut s = String::new();

//   match f.read_to_string(&mut s) {
//     Ok(_) => Ok(s),
//     Err(e) => Err(e),
//   }
// }

// Result 值之后的 ? 被定义为与上面的处理 Result 值的 match 表达式有着完全相同的工作方式。
// 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
// 如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，
// 这样错误值就被传播给了调用者

// match 表达式与问号运算符所做的有一点不同：? 运算符所使用的错误值被传递给了 from 函数，
// 它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型
// 当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型
// 这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败
// 只要每一个错误类型都实现了 from 函数来定义如何将自身转换为返回的错误类型，? 运算符会自动处理这些转换

fn read_username_from_file() -> Result<String, io::Error> {
  // let mut f = File::open("hello.txt")?;
  // let mut s = String::new();
  // f.read_to_string(&mut s)?;
  // Ok(s)

  // let mut s = String::new();
  // File::open("hello.txt")?.read_to_string(&mut s)?;
  // Ok(s)

  fs::read_to_string("hello.txt")
}
