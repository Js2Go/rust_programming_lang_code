use std::fmt::Display;

fn main() {
  // 生命周期与引用有效性
  // Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域
  // 大部分时候生命周期是隐含并可以推断的，正如大部分时候类型也是可以推断的一样
  // 类似于当因为有多种可能类型的时候必须注明类型，也会出现引用的生命周期以一些不同方式相关联的情况，
  // 所以 Rust 需要我们使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对是有效的

  // 生命周期避免了悬垂引用
  // 生命周期的主要目标是避免悬垂引用，它会导致程序引用了非预期引用的数据
  // {
  //   let r;
  //   {
  //     let x = 5;
  //     r = &x;
  //   }
  //   println!("r: {}", r);
  // }

  // 借用检查器
  // Rust 编译器有一个 借用检查器（borrow checker），它比较作用域来确保所有的借用都是有效的
  // {
  //   let r;                // ---------+-- 'a
  //                         //          |
  //   {                     //          |
  //     let x = 5;          // -+-- 'b  |
  //     r = &x;             //  |       |
  //   }                     // -+       |
  //                         //          |
  //   println!("r: {}", r); //          |
  // }                       // ---------+

  // 函数中的泛型生命周期
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
  
  // 生命周期注解语法
  // 生命周期注解并不改变任何引用的生命周期的长短。
  // 与当函数签名中指定了泛型类型参数后就可以接受任何类型一样，当指定了泛型生命周期后函数也能接受任何生命周期的引用。
  // 生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期
  
  // 生命周期注解有着一个不太常见的语法：生命周期参数名称必须以撇号（'）开头，其名称通常全是小写，类似于泛型其名称非常短
  // 'a 是大多数人默认使用的名称。生命周期参数注解位于引用的 & 之后，并有一个空格来将引用类型与生命周期注解分隔开
  
  // 单个的生命周期注解本身没有多少意义，因为生命周期注解告诉 Rust 多个引用的泛型生命周期参数如何相互联系的
  // &i32
  // &'a i32
  // &'a mut i32
  
  // 通过生命周期参数告诉Rust：longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致
  // let result;
  // {
  //   let string2 = String::from("xyz");
  //   result = longest(string1.as_str(), string2.as_str());
  // }
  // println!("The longest string is {}", result);

  // 深入理解生命周期
  // 指定生命周期参数的正确方式依赖函数实现的具体功能

  // 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配
  // 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，
  // 它将会是一个悬垂引用，因为它将会在函数结束时离开作用域

  // 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的
  // 一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为

  // 结构体定义中的生命周期注解
  // 这里的 main 函数创建了一个 ImportantExcerpt 的实例，
  // 它存放了变量 novel 所拥有的 String 的第一个句子的引用。
  // novel 的数据在 ImportantExcerpt 实例创建之前就存在。
  // 另外，直到 ImportantExcerpt 离开作用域之后 novel 都不会离开作用域，
  // 所以 ImportantExcerpt 实例中的引用是有效的
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentense = novel.split('.')
      .next()
      .expect("Could not find a '.'");
  let i = ImportantExcerpt { part: first_sentense };
  println!("{:?}", i);

  // 生命周期省略（Lifetime Elision）
  // 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），
  // 而返回值的生命周期被称为 输出生命周期（output lifetimes）


  // 编译器采用三条规则来判断引用何时不需要明确的注解。
  // 第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。
  // 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。
  // 这些规则适用于 fn 定义，以及 impl 块

  // 第一条规则是每一个是引用的参数都有它自己的生命周期参数。
  // 换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，
  // 有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推

  // 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32

  // 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
  // 说明是个对象的方法(method)(译者注： 这里涉及rust的面向对象参见17章), 那么所有输出生命周期参数被赋予 self 的生命周期。
  // 第三条规则使得方法更容易读写，因为只需更少的符号


  // 方法定义中的生命周期注解
  // （实现方法时）结构体字段的生命周期必须总是在 impl 关键字之后声明并在结构体名称之后被使用，因为这些生命周期是结构体类型的一部分
  // impl 块里的方法签名中，引用可能与结构体字段中的引用相关联，也可能是独立的。
  // 另外，生命周期省略规则也经常让我们无需在方法签名中使用生命周期注解


  // 静态生命周期
  // 'static，其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样标注出来
  let s: &'static str = "I have a static lifetime.";


  // 结合泛型类型参数、trait bounds 和生命周期
  longest_with_an_announcement("123", "45678", 123);
}

// 函数签名中的生命周期注解
// 就像泛型类型参数，泛型生命周期参数需要声明在函数名和参数列表间的尖括号中
// 这里我们想要告诉 Rust 关于参数中的引用和返回值之间的限制是他们都必须拥有相同的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
  x
}

// 这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
#[derive(Debug)]
struct ImportantExcerpt<'a> {
  part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
    3
  }
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
    {
      println!("Announcement! {}", ann);
      if x.len() > y.len() {
        x
      } else {
        y
      }
    }
