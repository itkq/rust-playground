use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

// struct BlockingQueue<T> {
//     tx: mpsc::SyncSender<T>,
//     rx: mpsc::Receiver<T>,
//     size: usize,
// }
//
// impl<T> BlockingQueue<T> {
//     fn new(size: usize) -> BlockingQueue<T> {
//         let (tx, rx) = mpsc::sync_channel(size);
//         BlockingQueue {
//             tx: tx,
//             rx: rx,
//             size: size,
//         }
//     }
//
//     fn get(&mut self) -> T {
//         self.rx.recv().unwrap()
//     }
//
//     fn put(&mut self, x: T) {
//         let _ = self.tx.send(x);
//     }
// }
//
// struct Producer<T> {
// }
//
// impl<T> Producer<T> {
//     fn new() -> Producer<T> {
//         Producer { }
//     }
//
//     fn run() {
//         loop {
//             // put
//         }
//     }
// }

// struct Consumer<T> {
//     queue: BlockingQueue<T>,
// }
//
// impl<T> Consumer<T> {
//     fn new(queue: BlockingQueue<T>) -> Consumer<T> {
//         Consumer { queue: queue }
//     }
//
//     fn run() {
//         loop {
//             // get
//         }
//     }
// }

struct Producer<T> {
    generator: fn() -> T,
}

impl<T> Producer<T> {
    fn new(generator: fn() -> T) -> Producer<T> {
        Producer {
            generator: generator,
        }
    }

    fn run(&self, tx: &Arc<Mutex<mpsc::SyncSender<T>>>) {
        let tx = tx.clone();
        let v = (self.generator)();
        thread::spawn(move || loop {
            let mut tx = tx.lock().unwrap();
            tx.send(v);
        });
    }
}

fn generator() -> i32 {
    1
}

fn main() {
    let (tx, rx) = mpsc::sync_channel(10);

    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));

    let p1 = Producer::new(generator);
    p1.run(&tx);

    // let mut queue = Arc::new(BlockingQueue::new(10));

    // let p1: Producer<i32> = Producer::new(queue.clone());
    // let p2: Producer<i32> = Producer::new(&queue);
    // let p3: Producer<i32> = Producer::new(&queue);

    // let c1: Consumer<i32> = Consumer::new(queue);
    // let c2: Consumer<i32> = Consumer::new(queue);
    // let c3: Consumer<i32> = Consumer::new(queue);
}
