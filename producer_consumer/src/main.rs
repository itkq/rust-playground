use std::sync::mpsc;
use std::sync::{Arc, Mutex};

struct BlockingQueue<T> {
    tx: mpsc::SyncSender<T>,
    rx: mpsc::Receiver<T>,
    size: usize,
}

impl<T> BlockingQueue<T> {
    fn new(size: usize) -> BlockingQueue<T> {
        let (tx, rx) = mpsc::sync_channel(size);
        BlockingQueue {
            tx: tx,
            rx: rx,
            size: size,
        }
    }

    fn get(&mut self) -> T {
        self.rx.recv().unwrap()
    }

    fn put(&mut self, x: T) {
        let _ = self.tx.send(x);
    }
}

struct Producer<T> {
    queue: &BlockingQueue<T>,
}

impl<T> Producer<T> {
    fn new(queue: &BlockingQueue<T>) -> Producer<T> {
        Producer { queue: queue }
    }

    fn run() {
        loop {
            // put
        }
    }
}

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

fn main() {
    let mut queue = BlockingQueue::new(10);

    let p1: Producer<i32> = Producer::new(&queue);
    let p2: Producer<i32> = Producer::new(&queue);
    let p3: Producer<i32> = Producer::new(&queue);

    // let c1: Consumer<i32> = Consumer::new(queue);
    // let c2: Consumer<i32> = Consumer::new(queue);
    // let c3: Consumer<i32> = Consumer::new(queue);
}
