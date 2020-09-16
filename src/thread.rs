use std::thread;
use std::time::Duration;

fn main() {
  // 无畏并发

  // 并发编程（Concurrent programming），代表程序的不同部分相互独立的执行，
  // 而 并行编程（parallel programming）代表程序不同部分于同时执行，
  // 这两个概念随着计算机越来越多的利用多处理器的优势时显得愈发重要


  // 使用线程同时运行代码

  // 使用 spawn 创建新线程
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  // 使用 join 等待所有线程结束
  // handle.join().unwrap();

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  // 使用 join 等待所有线程结束
  handle.join().unwrap();

  // 线程与 move 闭包
  let v = vec![1, 2, 3];
  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });
  // drop(v);
  handle.join().unwrap();
}
