use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct IndexOutOfBounds;

impl fmt::Display for IndexOutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index is out of bounds")
    }
}

impl Error for IndexOutOfBounds {}

mod array {
    use super::IndexOutOfBounds;

    pub struct Array<T> {
        pub arr: Vec<T>,
        capacity: usize,
        len: usize,
    }

    impl<T: Default + Copy + PartialEq> Array<T> {
        pub fn new(capacity: usize) -> Self {
            let mut arr = Vec::with_capacity(capacity);
            arr.resize(capacity, T::default()); 

            Array {
                arr,
                capacity,
                len: 0
            }
        }

        pub fn default() -> Self {
            Array::new(16) // or any default capacity you choose
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            self.arr.get(index)
        }

        pub fn set(& mut self, element: T, index: usize) -> Result<(), IndexOutOfBounds> {
            if index < self.capacity {
                Err(IndexOutOfBounds)
            } else {
                self.arr[index] = element;
                Ok(())
            }
        }

        pub fn clear(& mut self) {
            for index in 0..self.capacity {
                self.arr[index] = T::default();
                self.len = 0;
            }
        }

        pub fn add(& mut self, new_item: T) {            
            if self.len + 1 >= self.capacity {
                self.capacity *= 2;
                let mut new_arr = Vec::with_capacity(self.capacity);
                new_arr.resize(self.capacity, T::default());        

                for index in 0..self.capacity {
                    match self.get(index) {
                        Some(item) => new_arr[index] = *item,
                        None => new_arr[index] = T::default()
                    };
                }

                self.arr = new_arr;
            }

            
            self.arr[self.len] = new_item;
            self.len += 1;
        }

        pub fn remove_at(& mut self, rm_index: usize) -> Result<(), IndexOutOfBounds> {
            if rm_index >= self.len {
                return Err(IndexOutOfBounds);
            }

            let mut new_arr = Vec::with_capacity(self.capacity);
            new_arr.resize(self.capacity, T::default());

            let mut j = 0;
            for i in 0..self.len {
                if i != rm_index {
                    new_arr[j] = self.arr[i].clone();
                    j += 1;
                }
            }

            self.arr = new_arr;
            self.len -= 1;
            Ok(())
        }

        pub fn remove(&mut self) -> Result<(), IndexOutOfBounds> {
            if self.len == 0 {
                return Err(IndexOutOfBounds);
            }

            println!("{}", self.len);

            self.arr[self.len] = T::default();
            self.len -= 1;
            Ok(())
        }

        pub fn index(&self, item: T) -> isize {
            for index in 0..self.len {
                if item == self.arr[index] {
                    return index as isize;
                }
            }

            -1
        }
    }
}

use array::Array;

fn main() {
    let mut array = Array::<i32>::new(2);
    array.add(0);
    array.add(1);
    array.add(2);
    array.add(3);

    match array.remove() {
        Ok(()) => (),
        Err(e) => println!("{}", e)
    };
    array.add(4);

    
    println!("{:?}, {}", array.arr, array.index(1));
}