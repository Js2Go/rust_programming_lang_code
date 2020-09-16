use std::sync::{Mutex, Arc};
use std::thread;
// use std::rc::Rc;

fn main() {
  // 共享状态并发

  // 互斥器一次只允许一个线程访问数据
  // 互斥器（mutex）是 mutual exclusion 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据。
  // 为了访问互斥器中的数据，线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据。
  // 锁是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权。
  // 因此，我们描述互斥器为通过锁系统 保护（guarding）其数据

  // Mutex<T>的 API
  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }

  println!("m = {:?}", m);

  // 在线程间共享 Mutex<T>
  // 多线程和多所有权
  // 原子引用计数 Arc<T>

  // 所幸 Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。
  // 字母 “a” 代表 原子性（atomic），所以这是一个原子引用计数
  
  // let counter = Mutex::new(0);
  // let counter = Rc::new(Mutex::new(0));
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
  println!("Result: {}", *counter.lock().unwrap());
}
