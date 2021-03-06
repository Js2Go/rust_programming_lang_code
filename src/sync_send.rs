fn main() {
  // 使用 Sync 和 Send trait 的可扩展并发
  // Rust 的并发模型中一个有趣的方面是：语言本身对并发知之 甚少

  // 通过 Send 允许在线程间转移所有权
  // Send 标记 trait 表明类型的所有权可以在线程间传递。
  // 几乎所有的 Rust 类型都是Send 的，不过有一些例外，包括 Rc<T>

  // Sync 允许多线程访问
  // Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。
  // 换一种方式来说，对于任意类型 T，如果 &T（T 的引用）是 Send 的话 T 就是 Sync 的，
  // 这意味着其引用就可以安全的发送到另一个线程。
  // 类似于 Send 的情况，基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的

  // 手动实现 Send 和 Sync 是不安全的
  // 通常并不需要手动实现 Send 和 Sync trait，因为由 Send 和 Sync 的类型组成的类型，自动就是 Send 和 Sync 的
}
