use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  // 使用消息传递在线程间传送数据
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
  });

  let received = rx.recv().unwrap();
  println!("Got: {}", received);

  // 通道与所有权转移
  // let (tx, rx) = mpsc::channel();

  // thread::spawn(move || {
  //   let val = String::from("hi");
  //   tx.send(val).unwrap();
  //   println!("val is {}", val);
  // });

  // let received = rx.recv().unwrap();
  // println!("Got: {}", received);

  // 发送多个值并观察接收者的等待
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }

  // 通过克隆发送者来创建多个生产者
  let (tx, rx) = mpsc::channel();
  let tx1 = mpsc::Sender::clone(&tx);
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}
