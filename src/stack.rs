pub struct Stack {
    max: usize,
    curr: usize,
    arr: [i32; 100]
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            max: 100,
            curr: 0,
            arr: [0; 100]
        }
    }
    pub fn push(&mut self, elem: i32) -> bool {
        if self.curr == self.max {
            false
        } else {
            self.arr[self.curr] = elem;
            self.curr+=1;
            true
        }
    }
    pub fn pop(&mut self) -> Option<i32> {
        if self.curr == 0 {
            None
        } else {
            self.curr-=1;
            Some(self.arr[self.curr])
        }
    }
    pub fn top(&self) -> i32 {
        self.arr[self.curr-1]
    }
    pub fn empty(&self) -> bool {
        self.curr == 0
    }
    pub fn count(&self) -> usize {
        self.curr
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();

        for i in 1..101 {
            stack.push(i);
        }

        assert!(stack.top() == 100);
        assert_eq!(false, stack.push(1234));
        assert!(stack.count() == 100);
        assert!(stack.pop().unwrap() == 100);
        assert!(stack.top() == 99);

        for _ in 1..100 {
            stack.pop();
        }

        assert_eq!(None, stack.pop());
        assert!(stack.empty());
    }
}