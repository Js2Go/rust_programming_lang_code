use std::ops::Add;
use std::fmt;

fn main() {
  // 高级 trait
  // 关联类型在 trait 定义中指定占位符类型
  // 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型
  // trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型
  // 如此可以定义一个使用多种类型的 trait，直到实现此 trait 时都无需知道这些类型具体是什么
  pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
  }

  
  // 默认泛型类型参数和运算符重载
  // 当使用泛型类型参数时，可以为泛型指定一个默认的具体类型。如果默认类型就足够的话，这消除了为具体类型实现 trait 的需要
  // 为泛型类型指定默认类型的语法是在声明泛型类型时使用 <PlaceholderType=ConcreteType>
  // 这种情况的一个非常好的例子是用于运算符重载
  // 运算符重载（Operator overloading）是指在特定情况下自定义运算符（比如 +）行为的操作

  // Add trait定义
  // RHS=Self：这个语法叫做 默认类型参数（default type parameters）
  // RHS 是一个泛型类型参数（“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数
  // 如果实现 Add trait 时不指定 RHS 的具体类型，RHS 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型
  // trait Add<RHS=Self> {
  //   type Output;

  //   fn add(self, rhs: RHS) -> Self::Output;
  // }
  #[derive(Debug, PartialEq)]
  struct Point {
    x: i32,
    y: i32,
  }
  impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
      Point {
        x: self.x + other.x,
        y: self.y + other.y,
      }
    }
  }

  assert_eq!(Point {x: 1, y: 0} + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });

  // 为了使 Millimeters 和 Meters 能够相加，我们指定 impl Add<Meters> 来设定 RHS 类型参数的值而不是使用默认的 Self
  // 默认参数类型主要用于如下两个方面：
  // - 扩展类型而不破坏现有代码
  // - 在大部分用户都不需要的特定情况进行自定义

  #[derive(Debug)]
  struct Millimeters(u32);
  struct Meters(u32);
  impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
      Millimeters(self.0 + (other.0 * 1000))
    }
  }

  let mm = Millimeters(12);
  let m = Meters(12);
  println!("{:?}", mm + m);


  // 完全限定语法与消歧义：调用相同名称的方法
  // Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，也不能阻止为同一类型同时实现这两个 trait
  // 甚至直接在类型上实现开始已经有的同名方法也是可能的
  trait Pilot {
    fn fly(&self);
  }
  trait Wizard {
    fn fly(&self);
  }
  struct Human;
  impl Pilot for Human {
    fn fly(&self) {
      println!("This is your captain speaking.");
    }
  }
  impl Wizard for Human {
    fn fly(&self) {
      println!("Up!");
    }
  }
  impl Human {
    fn fly(&self) {
      println!("*waving arms furiously*");
    }
  }
  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();

  trait Animal {
    fn baby_name() -> String;
  }
  struct Dog;
  impl Dog {
    fn baby_name() -> String {
      String::from("Spot")
    }
  }
  impl Animal for Dog {
    fn baby_name() -> String {
      String::from("puppy")
    }
  }
  println!("A baby dog is called a {}", Dog::baby_name());
  // 为了消歧义并告诉 Rust 我们希望使用的是 Dog 的 Animal 实现，需要使用 完全限定语法，这是调用函数时最为明确的方式
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

  // 通常，完全限定语法定义为：
  // <Type as Trait>::function(receiver_if_method, next_arg, ...);


  // 父 trait 用于在另一个 trait 中使用某 trait 的功能
  // 有时我们可能会需要某个 trait 使用另一个 trait 的功能
  // 在这种情况下，需要能够依赖相关的 trait 也被实现。
  // 这个所需的 trait 是我们实现的 trait 的 父（超） trait（supertrait）
  trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();
      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2));
      println!("{}", "*".repeat(len + 4));
    }
  }
  struct Point2 {
    x: i32,
    y: i32,
  }
  impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
    }
  }
  impl OutlinePrint for Point2 {}
  let p = Point2 { x: 2, y : 4 };
  p.outline_print();


  // newtype 模式用以在外部类型上实现外部 trait
  // 孤儿规则（orphan rule），它说明只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait
  // 一个绕开这个限制的方法是使用 newtype 模式（newtype pattern），它涉及到在一个元组结构体中创建一个新类型
  // 这个元组结构体带有一个字段作为希望实现 trait 的类型的简单封装。接着这个封装类型对于 crate 是本地的，这样就可以在这个封装上实现 trait
  struct Wrapper(Vec<String>);
  impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "[{}]", self.0.join(", "))
    }
  }
  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}
