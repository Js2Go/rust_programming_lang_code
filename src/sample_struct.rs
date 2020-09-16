fn main() {
  // 一个使用结构体的示例程序
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels",
    area(width1, height1)
  );
  
  // 使用元组重构
  let rect = (30, 50);
  println!(
    "The area of the rectangle is {} square pixels",
    area2(rect)
  );
    
  // 使用结构体重构：赋予更多意义
  let rect = Rectangle {
    width: 30,
    height: 50,
  };
  println!(
    "The area of the rectangle is {} square pixels",
    area3(&rect)
  );

  // 通过派生 trait 增加实用功能
  // 在 {} 中加入 :? 指示符告诉 println! 我们想要使用叫做 Debug 的输出格式
  println!("{:?}", rect);
  // 使用 {:#?} 替换 println! 字符串中的 {:?}
  println!("{:#?}", rect);
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn area(w: u32, h: u32) -> u32 {
  w * h
}

fn area2(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}
