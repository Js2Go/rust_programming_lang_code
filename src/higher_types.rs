fn main() {
  // 高级类型
  // 为了类型安全和抽象而使用 newtype 模式
  

  // 类型别名用来创建类型同义词
  // 连同 newtype 模式，Rust 还提供了声明 类型别名（type alias）的能力，使用 type 关键字来给予现有类型另一个名字
  // 这意味着 Kilometers 是 i32 的 同义词（synonym）
  // Kilometers 不是一个新的、单独的类型。Kilometers 类型的值将被完全当作 i32 类型值来对待
  // 但通过这种手段无法获得上一部分讨论的 newtype 模式所提供的类型检查的好处
  type Kilometers = i32;
  let x: i32 = 5;
  let y: Kilometers = 5;
  println!("x + y = {}", x + y);

  let f: Thunk = Box::new(|| println!("hi"));

  // 动态大小类型和 Sized trait
}

type Thunk = Box<dyn Fn() + Send + 'static>;

type Result<T> = std::result::Result<T, std::io::Error>;

fn takes_long_type(f: Thunk) {
  let a: Result<i32>;
}

// 从不返回的 never type
// Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值
// 我们更倾向于称之为 never type。这个名字描述了它的作用：在函数从不返回的时候充当返回值
// 这读 “函数 bar 从不返回”，而从不返回的函数被称为 发散函数（diverging functions）
// 不能创建 ! 类型的值，所以 bar 也不可能返回值
// 描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型
// 允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值
// 相反它把控制权交回上层循环
// continue 宏 loop
fn bar() -> ! {
  println!("hhh")
}
