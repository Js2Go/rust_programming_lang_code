fn main() {
  // Rust 的面向对象编程特性
  // 为使用不同类型的值而设计的 trait 对象
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ]
      }),
      Box::new(Button {
        width: 50,
        height:10,
        label: String::from("OK"),
      })
    ]
  };
  screen.run();

  // Trait 对象要求对象安全
  // 只有 对象安全（object safe）的 trait 才可以组成 trait 对象
  // 如果一个 trait 中所有的方法有如下属性时，则该 trait 是对象安全的：
  // - 返回值类型不为 Self
  // - 方法没有任何泛型类型参数

  // Self 关键字是我们要实现 trait 或方法的类型的别名
  // 对象安全对于 trait 对象是必须的
}

pub trait Draw {
  fn draw(&self);
}
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}
impl Draw for Button {
  fn draw(&self) {
    // 实际绘制按钮的代码
  }
}

struct SelectBox {
  pub width: u32,
  pub height: u32,
  pub options: Vec<String>,
}
impl Draw for SelectBox {
  fn draw(&self) {
    // code to actually draw a select box
  }
}
