#![allow(dead_code, unused_variables)]
// Dynamic queue
struct Queue<T>(Vec<T>);

impl <T> Queue<T> {
    fn new() -> Self {
        Queue(Vec::new())
    }

    fn enqueue(&mut self, val: T) {
        self.0.push(val);
    }

    fn peek(&self) -> Option<&T> {
        if !self.is_empty() {
            Some(&self.0[0])
        } else {
            None
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        if !self.is_empty() {
            Some(self.0.remove(0))
        } else {
            None
        }
    }
    
    fn size(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[test]
fn enqueue_dequeue() {
    let mut queue = Queue::new();
    queue.enqueue(5);
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(9);

    assert_eq!(queue.dequeue(), Some(5));
    assert_eq!(queue.peek(), Some(&1));
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.peek(), Some(&9));
    assert_eq!(queue.dequeue(), Some(9));
    assert_eq!(queue.dequeue(), None);
}

#[derive(Debug)]
struct StaticQueue<T, const N: usize> {
    list: [T; N],
    size: usize
}

impl <T: Default + Copy, const N: usize> StaticQueue<T, N> {
    fn new() -> Self {
        StaticQueue {
            list: [T::default(); N],
            size: 0
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn enqueue(&mut self, val: T) {
        if self.size < N {
            self.list[self.size] = val;
            self.size += 1;
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        if !self.is_empty() {
            let mut new_list = [T::default(); N];

            for index in 0..N-1 {
                new_list[index] = self.list[index + 1];
            }

            let to_return = self.list[0];
            self.list = new_list;
            self.size -= 1;
            Some(to_return)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        if !self.is_empty() {
            Some(&self.list[0])
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[test]
fn static_enqueue_dequeue() {
    let mut queue = StaticQueue::<i32, 4>::new();
    queue.enqueue(5);
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(9);
    queue.enqueue(11);

    assert_eq!(queue.dequeue(), Some(5));
    assert_eq!(queue.peek(), Some(&1));
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.peek(), Some(&9));
    assert_eq!(queue.dequeue(), Some(9));
    assert_eq!(queue.dequeue(), None);
}
