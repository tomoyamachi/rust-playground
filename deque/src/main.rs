use std::collections::VecDeque;

#[derive(Debug)]
pub struct RestrectedVecDeque<T> {
    vec_deque: VecDeque<T>,
    max_len: usize,
}

impl<T> RestrectedVecDeque<T> {
    pub fn with_capacity(cap: usize) -> Self {
        RestrectedVecDeque {
            vec_deque: VecDeque::with_capacity(cap),
            max_len: cap,
        }
    }

    pub fn is_full(&self) -> bool {
        self.len() >= self.max_len
    }

    pub fn len(&self) -> usize {
        self.vec_deque.len()
    }

    pub fn push_back(&mut self, value: T) -> Option<T> {
        if self.max_len == 0 {
            return Some(value);
        }
        let popped = match self.is_full() {
            true => self.vec_deque.pop_front(),
            false => None
        };
        self.vec_deque.push_back(value);
        popped
    }
}

#[test]
fn test_std_deq() {
    let mut deq: VecDeque<u32> = VecDeque::with_capacity(10);
    for i in 0..5 {
        deq.push_back(i);
    }
    assert_eq!(deq.len(), 5);
    for i in 0..5 {
        deq.push_back(i);
    }
    assert_eq!(deq.len(), 10);
    for i in 0..5 {
        deq.push_back(i);
    }
    assert_eq!(deq.len(), 15);
}

#[test]
fn test_orginal_deq() {
    let mut deq: RestrectedVecDeque<u32> = RestrectedVecDeque::with_capacity(10);
    for i in 0..5 {
        deq.push_back(i);
    }
    assert_eq!(deq.len(), 5);
    for i in 0..5 {
        deq.push_back(i);
    }
    assert_eq!(deq.len(), 10);
    for i in 0..5 {
        deq.push_back(i);
    }
    assert_eq!(deq.len(), 10);
}
