#[derive(Debug)]
enum List {
  Cons(i32, Box<List>),
  Nil,
}

use crate::List::{Cons, Nil};

fn main() {
  // 使用 Box<T> 在堆上储存数据
  let b = Box::new(5);
  println!("b = {}", b);

  // Box 允许创建递归类型
  // let list = Cons(1, Cons(2, Cons(3, Nil)));

  // 使用 Box<T> 给递归类型一个已知的大小
  let list = Cons(1,
    Box::new(Cons(2,
      Box::new(Cons(3,
        Box::new(Nil))))));
  println!("list: {:?}", list);
}

