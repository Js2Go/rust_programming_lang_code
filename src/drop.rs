fn main() {
  // 使用 Drop Trait 运行清理代码
  // 对于智能指针模式来说第二个重要的 trait 是 Drop，其允许我们在值要离开作用域时执行一些代码
  let c = CustomSmartPointer { data: String::from("my stuff") };
  let d = CustomSmartPointer { data: String::from("other stuff") };
  println!("CustomSmartPointers created.");
  
  // 通过 std::mem::drop 提早丢弃值
  // 析构函数 对应创建实例的 构造函数。Rust 中的 drop 函数就是这么一个析构函数
  // std::mem::drop 函数不同于 Drop trait 中的 drop 方法。可以通过传递希望提早强制丢弃的值作为参数
  let c = CustomSmartPointer { data: String::from("my stuff") };
  println!("CustomSmartPointers created.");
  drop(c);
  println!("CustomSmartPointer dropped before the end of main.");

}

struct CustomSmartPointer {
  data: String,
}
impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}
