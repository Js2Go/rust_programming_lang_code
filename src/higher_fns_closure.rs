fn main() {
  // 高级函数与闭包
  // 函数指针
  // 函数的类型是 fn （使用小写的 “f” ）以免与 Fn 闭包 trait 相混淆。fn 被称为 函数指针（function pointer）
  // 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数
  let answer = do_twice(add_one, 5);

  println!("The answer is: {}", answer);


  // 返回闭包
  // 闭包表现为 trait，这意味着不能直接返回闭包
  // 对于大部分需要返回 trait 的情况，可以使用实现了期望返回的 trait 的具体类型来替代函数的返回值
  // 但是这不能用于闭包，因为他们没有一个可返回的具体类型
  // 例如不允许使用函数指针 fn 作为返回值类型
  returns_closure();
}

fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

// fn returns_closure() -> Fn(i32) -> i32 {
//   |x| x + 1
// }
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}
