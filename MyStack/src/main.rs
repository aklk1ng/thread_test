#[derive(Debug)]
struct MyStack {
    queue: Vec<i32>,
}
impl MyStack {
    fn new() -> Self {
        MyStack { queue: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.queue.push(x);
    }

    fn pop(&mut self) -> i32 {
        let len = self.queue.len() - 1;
        for _ in 0..len {
            let tmp = self.queue.remove(0);
            self.queue.push(tmp);
        }
        self.queue.remove(0)
    }

    fn top(&mut self) -> i32 {
        let res = self.pop();
        self.queue.push(res);
        res
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn main() {
    println!("Hello, world!");
}
