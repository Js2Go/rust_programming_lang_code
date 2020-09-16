use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
  // RefCell<T> 和内部可变性模式
  // 内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的
  // 为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则


  // 通过 RefCell<T> 在运行时检查借用规则

  // 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
  // - Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
  // - Box<T> 允许在编译时执行不可变或可变借用检查；
  //   Rc<T>仅允许在编译时执行不可变借用检查；
  //   RefCell<T> 允许在运行时执行不可变或可变借用检查。
  // - 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值

  // 在不可变值内部改变值就是 内部可变性 模式


  // 内部可变性：不可变值的可变借用
  // let x = 5;
  // let y = &mut x;

  
  // 内部可变性的用例：mock 对象
  // 测试替身（test double）是一个通用编程概念，它代表一个在测试中替代某个类型的类型
  // mock 对象 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的


  // RefCell<T> 在运行时记录借用


  // 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

  *value.borrow_mut() += 10;

  println!("a after = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    sent_messenges: RefCell<Vec<String>>,
  }
  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger { sent_messenges: RefCell::new(vec![]) }
    }
  }
  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      self.sent_messenges.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messenges.borrow_mut().len(), 1);
  }
}

pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
  where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
      LimitTracker {
        messenger,
        value: 0,
        max,
      }
    }

    pub fn set_value(&mut self, value: usize) {
      self.value = value;

      let percentage_of_max = self.value as f64 / self.max as f64;

      if percentage_of_max >= 1.0 {
        self.messenger.send("Error: You are over your quota!");
      } else if percentage_of_max >= 0.9 {
        self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
      } else {
        self.messenger.send("Warning: You've used up over 75% of your quota!");
      }
    }
  }
