// threads3.rs
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a hint.



#[derive(Clone)]
struct Queue {
    data: Vec<i32>,
}

impl Queue {
    fn new() -> Queue {
        Queue { data: Vec::new() }
    }

    fn enqueue(&mut self, item: i32) {
        self.data.push(item);
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.data.pop()
    }
}

fn send_tx(queue: Queue, shared_tx: i32) {
    // Code for sending the transaction
}

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    let shared_tx = 123;
    send_tx(queue.clone(), shared_tx);
}
