#![allow(dead_code)]

use std::{collections::{BTreeSet, HashMap}, fmt::Debug, hash::Hash};

pub enum Sort {
    Min, 
    Max
}

pub struct PriorityQueue<T> {
    pub heap: Vec<T>,
    heap_size: usize,
    sort: Sort,
    pub map: HashMap<T, BTreeSet<usize>>
}

impl <T: Ord + Debug + Hash + Clone> PriorityQueue<T> {
    pub fn new(sort: Sort) -> Self {
        PriorityQueue {
            heap: vec![],
            heap_size: 0,
            sort,
            map: HashMap::new()
        }
    }

    pub fn add(&mut self, elem: T) {
        self.map.entry(elem.clone()).or_insert_with(BTreeSet::new).insert(self.heap_size);
        self.heap.push(elem);

        self.swim_up(self.heap_size);
        self.heap_size += 1;
    }

    // true if i is in the valid sorting order to j, otherwise false
    fn compare(&self, i: usize, j: usize) -> bool {
        let output = self.heap.get(i).zip(self.heap.get(j)).map_or(false, |(a, b)| a <= b);

        match self.sort {
            Sort::Min => output,
            Sort::Max => !output
        }
    }

    fn swap(&mut self, i: usize, j: usize) {        
        // Swapping in map
        let i_elem = self.heap[i].clone();
        let j_elem = self.heap[j].clone();
        self.heap.swap(i, j);

        if i_elem != j_elem {
            if let Some(i_set) = self.map.get_mut(&i_elem) {
                i_set.remove(&i);
                i_set.insert(j);
            }

            if let Some(j_set) = self.map.get_mut(&j_elem) {
                j_set.remove(&j);
                j_set.insert(i);
            }
        }
    }

    fn swim_up(&mut self, mut index: usize) {
        while index > 0  {
            let parent = (index - 1) / 2;

            if self.compare(index, parent) {
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

            if self.compare(right_child, left_child) {
                smaller = right_child;
            }

            if smaller > self.heap_size - 1 || self.compare(index, smaller) {
                break;
            }

            self.swap(index, smaller);
            index = smaller;
        }
    }

    pub fn size(&self) -> usize {
        self.heap_size
    }

    pub fn poll(&mut self) -> T {
        self.swap(0, self.heap_size - 1);
        let elem = self.heap.remove(self.heap_size - 1);
        self.heap_size -= 1;

        self.swim_down(0);

        elem
    }

    pub fn remove(&mut self, elem: T) {
        let index = self.get_index(elem.clone());
        
        if let Some(node_index) = index {
            self.swap(node_index, self.heap_size - 1);
            
            self.heap.remove(self.heap_size - 1);
            self.map_remove(elem, self.heap_size - 1);
            
            self.heap_size -= 1;
            self.swim_up(node_index);
        }
    }

    fn map_remove(&mut self, elem: T, index: usize) {
        if let Some(set) = self.map.get_mut(&elem) {
            set.remove(&index);
            println!("{:?} {}", set, index);
            if set.is_empty() {
                self.map.remove(&elem);
            }
        }
    }

    pub fn get_index(&self, elem: T) -> Option<usize> {
        self.map.get(&elem).map_or(None, |set| set.first().cloned())

        // for i in 0..self.heap_size {
        //     if self.heap[i] == elem {
        //         return Some(i);
        //     }
        // }

        // None
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

    assert_eq!(queue.poll(), 1);
    assert_eq!(queue.poll(), 2);
    assert_eq!(queue.poll(), 5);
    assert_eq!(queue.poll(), 11);
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

    assert_eq!(queue.poll(), 1);
    assert_eq!(queue.poll(), 2);
    assert_eq!(queue.poll(), 5);
    assert_eq!(queue.poll(), 11);
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

    assert_eq!(queue.poll(), -5);
    assert_eq!(queue.poll(), 0);
    assert_eq!(queue.poll(), 1);
    assert_eq!(queue.poll(), 1);
    assert_eq!(queue.poll(), 2);
    assert_eq!(queue.poll(), 3);
    assert_eq!(queue.poll(), 5);
    assert_eq!(queue.poll(), 11);
    assert_eq!(queue.poll(), 99);
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

    assert_eq!(queue.poll(), 11);
    assert_eq!(queue.poll(), 5);
    assert_eq!(queue.poll(), 2);
    assert_eq!(queue.poll(), 1);
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

    assert_eq!(queue.poll(), 11);
    assert_eq!(queue.poll(), 5);
    assert_eq!(queue.poll(), 2);
    assert_eq!(queue.poll(), 1);
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

    assert_eq!(queue.poll(), 99);
    assert_eq!(queue.poll(), 11);
    assert_eq!(queue.poll(), 5);
    assert_eq!(queue.poll(), 3);
    assert_eq!(queue.poll(), 2);
    assert_eq!(queue.poll(), 1);
    assert_eq!(queue.poll(), 1);
    assert_eq!(queue.poll(), 0);
    assert_eq!(queue.poll(), -5);
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

    assert_eq!(queue.poll(), -5);
    assert_eq!(queue.poll(), 1);
    assert_eq!(queue.poll(), 1);
    assert_eq!(queue.poll(), 2);
    assert_eq!(queue.poll(), 3);
    assert_eq!(queue.poll(), 5);
    assert_eq!(queue.poll(), 7);
    assert_eq!(queue.poll(), 11);
}

// Contains
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

    println!("{:?}", queue.heap);
    println!("{:?}", queue.map);

    assert_eq!(queue.contains(-5), false);
    queue.remove(11);
    assert_eq!(queue.contains(11), false);
}
