use std::fmt::Display;

fn main() {
  // trait：定义共享的行为
  // trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。
  // 可以通过 trait 以一种抽象的方式定义共享的行为。
  // 可以使用 trait bounds 指定泛型是任何拥有特定行为的类型

  // trait 类似于其他语言中的常被称为 接口（interfaces）的功能，虽然有一些不同

  // 定义 trait
  // 一个类型的行为由其可供调用的方法构成。
  // 如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
  // trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合

  // 为类型实现 trait
  // 实现 trait 时需要注意的一个限制是，
  // 只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait

  // 但是不能为外部类型实现外部 trait
  // 不能在 aggregator crate 中为 Vec<T> 实现 Display trait
  // 这是因为 Display 和 Vec<T> 都定义于标准库中，它们并不位于 aggregator crate 本地作用域中
  // 这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型
  // 这条规则确保了其他人编写的代码不会破坏你代码，反之亦然
  // 没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };
  println!("1 new tweet: {}", tweet.summarize());

  // 默认实现
  // 有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的
  // 这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为

  // 如果想要对 NewsArticle 实例使用这个默认实现，而不是定义一个自己的实现，
  // 则可以通过 impl Summary for NewsArticle {} 指定一个空的 impl 块

  // trait 作为参数
  // 定义一个函数 notify 来调用其参数 item 上的 summarize 方法，该参数是实现了 Summary trait 的某种类型。
  // 为此可以使用 impl Trait 语法
  notify(tweet);

  // Trait Bound 语法
  // impl Trait 语法适用于直观的例子，它不过是一个较长形式的语法糖。这被称为 trait bound
  // pub fn notify<T: Summary>(item1: T, item2: T) {}

  // 通过 + 指定多个 trait bound
  // 如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，
  // 那么 item 就需要同时实现两个不同的 trait：Display 和 Summary
  // pub fn notify(item: impl Summary + Display) {}
  // pub fn notify<T: Summary + Display>(item: T) {}

  // 通过 where 简化 trait bound
  // fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
  // fn some_function<T, U>(t: T, u: U) -> i32
  //   where T: Display + Clone,
  //         U: Clone + Debug
  //   {}

  // 返回实现了 trait 的类型
  // 也可以在返回值中使用 impl Trait 语法，来返回实现了某个 trait 的类型
  // 通过使用 impl Summary 作为返回值类型，
  // 我们指定了 returns_summarizable 函数返回某个实现了 Summary trait 的类型，但是不确定其具体的类型
  // 不过这只适用于返回单一类型的情况

  // fn return_summarizable() -> impl Summary {
  //   Tweet {
  //     username: String::from("horse_ebooks"),
  //     content: String::from("of course, as you probably already know, people"),
  //     reply: false,
  //     retweet: false,
  //   }
  // }

  // 使用 trait bound 有条件地实现方法
  // 通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法

}

pub trait Summary {
  // fn summarize(&self) -> String;
  fn summarize_author(&self) -> String {
    String::from("")
  }
  // 默认实现
  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}
impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
  // fn summarize(&self) -> String {
  //   format!("{}: {}", self.username, self.content)
  // }
}

fn notify(item: impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// 使用 trait bounds 来修复 largest 函数

// 在 largest 函数体中我们想要使用大于运算符（>）比较两个 T 类型的值。
// 这个运算符被定义为标准库中 trait std::cmp::PartialOrd 的一个默认方法。
// 所以需要在 T 的 trait bound 中指定 PartialOrd，这样 largest 函数可以用于任何可以比较大小的类型的 slice

// 像 i32 和 char 这样的类型是已知大小的并可以储存在栈上，所以他们实现了 Copy trait。
// 当我们将 largest 函数改成使用泛型后，现在 list 参数的类型就有可能是没有实现 Copy trait 的

// 如果并不希望限制 largest 函数只能用于实现了 Copy trait 的类型，
// 我们可以在 T 的 trait bounds 中指定 Clone 而不是 Copy。
// 并克隆 slice 的每一个值使得 largest 函数拥有其所有权。
// 使用 clone 函数意味着对于类似 String 这样拥有堆上数据的类型，会潜在的分配更多堆上空间，而堆分配在涉及大量数据时可能会相当缓慢

// 另一种 largest 的实现方式是返回在 slice 中 T 值的引用。
// 如果我们将函数返回值从 T 改为 &T 并改变函数体使其能够返回一个引用，我们将不需要任何 Clone 或 Copy 的 trait bounds 而且也不会有任何的堆分配
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

struct Pair<T> {
  x: T,
  y: T,
}
impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {
      x,
      y,
    }
  }
}
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

// 也可以对任何实现了特定 trait 的类型有条件地实现 trait。
// 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations，他们被广泛的用于 Rust 标准库中
// 标准库为任何实现了 Display trait 的类型实现了 ToString trait
// impl<T: Display> ToString for T {}
