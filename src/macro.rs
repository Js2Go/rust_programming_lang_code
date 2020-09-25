fn main() {
  // 宏
  // 宏（Macro）指的是 Rust 中一系列的功能：声明（Declarative）宏，使用 macro_rules!，和三种 过程（Procedural）宏：
  // - 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
  // - 类属性（Attribute-like）宏定义可用于任意项的自定义属性
  // - 类函数宏看起来像函数不过作用于作为参数传递的 token。

  
  // 宏和函数的区别
  // 宏是一种为写其他代码而写代码的方式，即所谓的 元编程（metaprogramming）
  // 一个函数标签必须声明函数参数个数和类型。
  // 相比之下，宏能够接受不同数量的参数：用一个参数调用 println!("hello") 或用两个参数调用 println!("hello {}", name)
  // 宏可以在编译器翻译代码前展开
  // 宏可以在一个给定类型上实现 trait 。而函数则不行，因为函数是在运行时被调用，同时 trait 需要在编译时实现
  // 宏和函数的最后一个重要的区别是：在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用


  // 使用 macro_rules! 的声明宏用于通用元编程
  // Rust 最常用的宏形式是 声明宏（declarative macros）
  // 它们有时也被称为 “macros by example”、“macro_rules! 宏” 或者就是 “macros”
  // 其核心概念是，声明宏允许我们编写一些类似 Rust match 表达式的代码
  // 宏也将一个值和包含相关代码的模式进行比较
  // 此种情况下，该值是传递给宏的 Rust 源代码字面值，模式用于和传递给宏的源代码进行比较，同时每个模式的相关代码则用于替换传递给宏的代码
  // 可以使用 macro_rules! 来定义宏。让我们通过查看 vec! 宏定义来探索如何使用 macro_rules! 结构
  let v: Vec<u32> = vec![1, 2, 3];
  // println!("{}", v);


  // 用于从属性生成代码的过程宏
  // 第二种形式的宏被称为 过程宏（procedural macros），因为它们更像函数（一种过程类型）。
  // 过程宏接收 Rust 代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出，而非像声明式宏那样匹配对应模式然后以另一部分代码替换当前代码
  // 有三种类型的过程宏（自定义派生（derive），类属性和类函数），不过它们的工作方式都类似
  // 当创建过程宏时，其定义必须位于一种特殊类型的属于它们自己的 crate 中。这么做出于复杂的技术原因，将来我们希望能够消除这些限制
  // use proc_macro;
  // #[some_attribute]
  // pub fn some_name(input: TokenStream) -> TokenStream {}


  // 如何编写自定义 derive 宏

  // 类属性宏

  // 类函数宏
}

#[macro_export]
macro_rules! vecc {
  // $() 之后的逗号说明一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后。紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
}
