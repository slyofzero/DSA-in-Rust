#![allow(dead_code)]

use std::fmt::Debug;

pub enum Sort {
    Min, 
    Max
}

pub struct PriorityQueue<T> {
    heap: Vec<T>,
    heap_size: usize,
    sort: Sort
}

impl <T: Ord + Debug> PriorityQueue<T> {
    pub fn new(sort: Sort) -> Self {
        PriorityQueue {
            heap: vec![],
            heap_size: 0,
            sort
        }
    }

    pub fn add(&mut self, elem: T) {
        self.heap.push(elem);
        self.swim_up(self.heap_size);
        self.heap_size += 1;
    }

    fn is_sorted(&self, i: usize, j: usize) -> bool {
        let output = self.heap.get(i).zip(self.heap.get(j)).map_or(false, |(a, b)| a <= b);

        match self.sort {
            Sort::Min => output,
            Sort::Max => !output
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        // println!("{i} {j}");
        self.heap.swap(i, j);
    }

    fn swim_up(&mut self, mut index: usize) {
        while index > 0  {
            let parent = (index - 1) / 2;

            if self.is_sorted(index, parent) {
                self.swap(parent, index);
            }

            index = parent;
        }
    }

    fn swim_down(&mut self, mut index: usize) {
        while self.heap_size > 0 {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            // Setting to left one by default in case of a tie
            let mut smaller = left_child;

            if self.is_sorted(right_child, left_child) {
                smaller = right_child;
            }

            if smaller > self.heap_size - 1 || self.is_sorted(index, smaller) {
                break;
            }

            self.swap(index, smaller);
            index = smaller;
        }
    }

    pub fn size(&self) -> usize {
        self.heap_size
    }

    pub fn pop(&mut self) -> T {
        self.remove_at(0)
    }

    pub fn remove(&mut self, elem: T) {
        let index = self.get_index(elem);
        index.map(|i| self.remove_at(i));
    }

    pub fn remove_at(&mut self, index: usize) -> T {
        self.swap(index, self.heap_size - 1);
        let elem = self.heap.remove(self.heap_size - 1);
        self.heap_size -= 1;

        self.swim_down(index);

        elem
    }

    pub fn get_index(&self, elem: T) -> Option<usize> {
        for i in 0..self.heap_size {
            if self.heap[i] == elem {
                return Some(i);
            }
        }

        None
    }

    pub fn contains(&self, elem: T) -> bool {
        let index = self.get_index(elem);
        match index {
            Some(_) => true,
            None => false
        }
    }
}

// Min Priority Queue
#[test]
fn add_descending() {
    let mut queue = PriorityQueue::<isize>::new(Sort::Min);

    queue.add(11);
    queue.add(5);
    queue.add(2);
    queue.add(1);

    // Should be
    // 1, 2, 5, 11

    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.pop(), 2);
    assert_eq!(queue.pop(), 5);
    assert_eq!(queue.pop(), 11);
}

#[test]
fn add_ascending() {
    let mut queue = PriorityQueue::<isize>::new(Sort::Min);

    queue.add(1);
    queue.add(2);
    queue.add(5);
    queue.add(11);

    // Should be
    // 1, 2, 5, 11

    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.pop(), 2);
    assert_eq!(queue.pop(), 5);
    assert_eq!(queue.pop(), 11);
}

#[test]
fn add_random() {
    let mut queue = PriorityQueue::<isize>::new(Sort::Min);

    queue.add(11);
    queue.add(1);
    queue.add(5);
    queue.add(2);
    queue.add(3);
    queue.add(99);
    queue.add(1);
    queue.add(-5);
    queue.add(0);

    // Should be
    // -5, 0, 1, 1, 2, 3, 5, 11, 99

    assert_eq!(queue.pop(), -5);
    assert_eq!(queue.pop(), 0);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.pop(), 2);
    assert_eq!(queue.pop(), 3);
    assert_eq!(queue.pop(), 5);
    assert_eq!(queue.pop(), 11);
    assert_eq!(queue.pop(), 99);
}

// Max Priority Queue
#[test]
fn max_add_descending() {
    let mut queue = PriorityQueue::<isize>::new(Sort::Max);

    queue.add(11);
    queue.add(5);
    queue.add(2);
    queue.add(1);

    // Should be
    // 11, 5, 2, 1

    assert_eq!(queue.pop(), 11);
    assert_eq!(queue.pop(), 5);
    assert_eq!(queue.pop(), 2);
    assert_eq!(queue.pop(), 1);
}

#[test]
fn max_add_ascending() {
    let mut queue = PriorityQueue::<isize>::new(Sort::Max);

    queue.add(1);
    queue.add(2);
    queue.add(5);
    queue.add(11);

    // Should be
    // 11, 5, 2, 1

    assert_eq!(queue.pop(), 11);
    assert_eq!(queue.pop(), 5);
    assert_eq!(queue.pop(), 2);
    assert_eq!(queue.pop(), 1);
}

#[test]
fn max_add_random() {
    let mut queue = PriorityQueue::<isize>::new(Sort::Max);

    queue.add(11);
    queue.add(1);
    queue.add(5);
    queue.add(2);
    queue.add(3);
    queue.add(99);
    queue.add(1);
    queue.add(-5);
    queue.add(0);

    // Should be
    // 99, 11, 5, 3, 2, 1, 1, 0, -5

    assert_eq!(queue.pop(), 99);
    assert_eq!(queue.pop(), 11);
    assert_eq!(queue.pop(), 5);
    assert_eq!(queue.pop(), 3);
    assert_eq!(queue.pop(), 2);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.pop(), 0);
    assert_eq!(queue.pop(), -5);
}

// Removal
#[test]
fn remove() {
    let mut queue = PriorityQueue::<isize>::new(Sort::Min);

    queue.add(11);
    queue.add(5);
    queue.add(2);
    queue.add(-5);
    queue.add(1);
    queue.add(3);
    queue.add(3);
    queue.add(7);
    queue.add(1);

    queue.remove(3);

    // Should be
    // -5, 1, 1, 2, 3, 5, 7, 11

    assert_eq!(queue.pop(), -5);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.pop(), 2);
    assert_eq!(queue.pop(), 3);
    assert_eq!(queue.pop(), 5);
    assert_eq!(queue.pop(), 7);
    assert_eq!(queue.pop(), 11);
}

// Removal
#[test]
fn contains() {
    let mut queue = PriorityQueue::<isize>::new(Sort::Min);

    queue.add(11);
    queue.add(5);
    queue.add(2);
    queue.add(-5);

    // Should be
    // -5, 2, 5, 11

    assert_eq!(queue.contains(-5), true);
    assert_eq!(queue.contains(1), false);
    queue.remove(-5);
    assert_eq!(queue.contains(-5), false);
    queue.remove(11);
    assert_eq!(queue.contains(11), false);
}
