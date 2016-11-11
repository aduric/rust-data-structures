#![allow(non_snake_case)]

pub struct Queue {
    N: usize,
    front: usize,
    rear: usize,
    arr: [i32; 100]
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            N: 100,
            front: 0,
            rear: 0,
            arr: [0; 100]
        }
    }
    pub fn enqueue(&mut self, elem: i32) -> bool {
        if self.count() == self.N  {
            false
        } else {
            self.arr[self.rear] = elem;
            self.rear = (self.rear + 1) % self.N;
            true
        }
    }
    pub fn dequeue(&mut self) -> Option<i32> {
        if self.empty() {
            None
        } else {
            let temp = self.front;
            self.front = (self.front + 1) % self.N;
            Some(self.arr[temp])
        }
    }
    pub fn front(&self) -> i32 {
        self.arr[self.front]
    }
    pub fn empty(&self) -> bool {
        self.rear == self.front
    }
    pub fn count(&self) -> usize {
        (self.N - self.front + self.rear) % self.N + 1
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();

        for i in 1..101 {
            queue.enqueue(i);
        }

        assert!(queue.front() == 1);
        assert_eq!(false, queue.enqueue(1234));
        assert!(queue.count() == 100);
        assert!(queue.dequeue().unwrap() == 1);
        assert!(queue.front() == 2);

        for _ in 1..100 {
            queue.dequeue();
        }

        assert_eq!(None, queue.dequeue());
        assert!(queue.empty());
    }
}