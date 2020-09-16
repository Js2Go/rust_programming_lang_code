fn main() {
  // 包和 crate
  // 包（package） 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件

  // 包中所包含的内容由几条规则来确立。
  // 一个包中至多 只能 包含一个库 crate(library crate)；包中可以包含任意多个二进制 crate(binary crate)；
  // 包中至少包含一个 crate，无论是库的还是二进制的

  // Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。
  // 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。
  // 这是因为子模块封装并隐藏了他们的实现详情，但是子模块可以看到他们定义的上下文
  crate::front::hosting::add_to_waitlist();
  front::hosting::add_to_waitlist();

  eat_at_restaurant()


  // 使用 use 关键字将名称引入作用域
  // use crate::front::hosting;

  // 对于函数
  // 创建惯用的 use 路径
  // 要想使用 use 将函数的父模块引入作用域，我们必须在调用函数时指定父模块，
  // 样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化
  // 使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径

  // 使用 as 关键字提供新的名称
  // 使用 use 将两个同名类型引入同一作用域这个问题还有另一个解决办法：
  // 在这个类型的路径后面，我们使用 as 指定一个新的本地名称或者别名
  // use std::fmt::Result;
  // use std::io::Result as IoResult;

  // 使用 pub use 重导出名称
  // 果为了让调用你编写的代码的代码能够像在自己的作用域内引用这些类型，可以结合 pub 和 use。
  // 这个技术被称为 “重导出（re-exporting）”，因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域
  // pub use crate::front::hosting;

  // 使用外部包

  // 嵌套路径来消除大量的 use 行
  // 当需要引入很多定义于相同包或相同模块的项时，为每一项单独列出一行会占用源码很大的空间
  // use std::cmp::Ordering;
  // use std::io;
  // use std::{cmp::Ordering, io};

  // use std::io;
  // use std::io::Write;
  // use std::io::{self, Write};

  // 通过 glob 运算符将所有的公有定义引入作用域
  // 如果希望将一个路径下 所有 公有项引入作用域，可以指定路径后跟 *，glob 运算符
  // 这个 use 语句将 std::collections 中定义的所有公有项引入当前作用域。
  // 使用 glob 运算符时请多加小心！Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的
  // glob 运算符经常用于测试模块 tests 中，这时会将所有内容引入作用域
  // use std::collections::*;


  // 将模块分割进不同文件
}

fn serve_order() {}

mod front {
  pub mod hosting {
    pub fn add_to_waitlist() {
      println!("pub pub pub");
      // 使用 super 起始的相对路径（相当于../）
      super::super::serve_order();
    }
  }
}

mod back_of_house {
  // 创建公有的结构体和枚举
  pub struct Breakfast {
    pub toast: String,
    _seasonal_fruit: String,
  }
  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        _seasonal_fruit: String::from("peaches"),
      }
    }
  }
  fn _fix_incorrect_order() {
    _cook_order();
    super::serve_order();
  }
  fn _cook_order() {}

  // 如果我们将枚举设为公有，则它的所有成员都将变为公有
  #[derive(Debug)]
  pub enum Apptizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  let order1 = back_of_house::Apptizer::Soup;
  let order2 = back_of_house::Apptizer::Salad;
  println!("I'd like {} toast please", meal.toast);
  println!("order1: {:?}, order2: {:?}", order1, order2);
}
