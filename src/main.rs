mod linked_list {
    use std::{cell::RefCell, fmt::Display, marker::PhantomData, rc::Rc};

    type NodePointer<T> = Rc<RefCell<Node<T>>>;

    #[derive(Debug)]
    pub struct Node<T> {
        val: T,
        next: Option<NodePointer<T>>
    }

    impl <T: Display> Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.val)
        }
    }

    impl <T> Node<T> {
        fn new(val: T) -> Self {
            Node { val, next: None }
        }
    }

    // ------------------------------ Node ------------------------------
    #[derive(Debug)]
    pub struct LinkedList<T> {
        head: Option<NodePointer<T>>,
        tail: Option<NodePointer<T>>,
        length: u32
    }

    impl <T: Display> Display for LinkedList<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(head_node_rc) = self.head.clone()  {
                write!(f, "{}", head_node_rc.borrow())?;
                let mut current = head_node_rc.borrow().next.clone();
                
                while let Some(node_rc) = current {
                    let node = node_rc.borrow();
                    write!(f, " -> {}", node.val)?;
                    current = node.next.clone();
                }
            }

            write!(f, "\nLength - {}", self.length)
        }
    }

    // ------------------------------ Linked List ------------------------------
    impl <T: Clone> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList { head: None, tail: None, length: 0 }
        }

        pub fn push(&mut self, val: T) {
            let new_node = Rc::new(RefCell::new(Node::new(val)));

            if let Some(tail_node_rc) = self.tail.clone() {
                tail_node_rc.borrow_mut().next = Some(new_node.clone());
            } else {
                self.head = Some(new_node.clone());
            }

            self.tail = Some(new_node.clone());
            self.length += 1;
        }

        pub fn insert_at(&mut self, val: T, insert_index: u32) {
            let new_node = Rc::new(RefCell::new(Node::new(val)));

            if insert_index == 0 {
                new_node.borrow_mut().next = self.head.clone();
                self.head = Some(new_node);
                self.length += 1;
                return;
            }

            let mut current_node = self.head.clone();
            let mut current_index = 1;

            while let Some(current_node_rc) = current_node {
                if current_index == insert_index {
                    new_node.borrow_mut().next = current_node_rc.borrow().next.clone();
                    current_node_rc.borrow_mut().next = Some(new_node.clone());
                    self.length += 1;
                    break;
                }

                current_index += 1;
                current_node = current_node_rc.borrow().next.clone();
            }
        }
    
        pub fn pop(&mut self) -> Option<T> {
            if let Some(head_node) = self.head.clone() {
                let new_head = head_node.borrow().next.clone();
                self.head = new_head;
                let pop_value = head_node.borrow().val.clone();
                Some(pop_value)
            } else {
                None
            }
        }

        pub fn peek(&self) -> Option<T> {
            if let Some(head_node) = self.head.clone() {
                let head_value = head_node.borrow().val.clone();
                Some(head_value)
            } else {
                None
            }
        }

        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }
    
        pub fn iter(&self) -> Iter<'_, T> {
            Iter { current_node: self.head.clone(), phantom: PhantomData }
        }

        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut { current_node: self.head.clone(), phantom: PhantomData }
        }

        pub fn get(&self, index: u32) -> Option<T> {
            let mut current_index = 0;

            for node in self.iter() {
                if current_index == index {
                    return Some(node.clone());
                }

                current_index += 1;
            }

            None
        }
    
        pub fn drop(&mut self, drop_index: u32) -> Option<T> {
            if drop_index == 0 {
                if let Some(head_node) = self.head.clone() {
                    let new_head = head_node.borrow().next.clone();
                    self.head = new_head;
                    return Some(head_node.borrow().val.clone());
                }
            }

            let mut current_node = self.head.clone();
            let mut current_index = 1;

            while let Some(current_node_rc) = current_node {
                if current_index == drop_index {
                    let node_to_drop = current_node_rc.borrow().next.clone();
                    
                    if let Some(node_to_drop_rc) = node_to_drop {
                        let next_node = node_to_drop_rc.borrow().next.clone();
                        current_node_rc.borrow_mut().next = next_node;
                        return Some(node_to_drop_rc.borrow().val.clone());
                    } else {
                        return None;
                    }
                }

                current_index += 1;
                current_node = current_node_rc.borrow().next.clone();
            }

            None
        }
    }

    // ------------------------------ Iterator ------------------------------
    pub struct IntoIter<T>(LinkedList<T>);

    impl <T: Clone> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    pub struct Iter<'a, T> {
        current_node: Option<NodePointer<T>>,
        phantom: PhantomData<&'a T>
    }

    impl <'a, T: Clone> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.current_node.clone().map(|node| {
                self.current_node = node.borrow().next.clone();
                unsafe { &*(&node.borrow().val as *const T) }
            })
        }
    }

    pub struct IterMut<'a, T> {
        current_node: Option<NodePointer<T>>,
        phantom: PhantomData<&'a T>
    }

    impl <'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.current_node.clone().map(|node| {
                self.current_node = node.borrow().next.clone();
                unsafe { &mut *(&mut node.borrow_mut().val as *mut T) }
            })
        }
    }
}

use linked_list::LinkedList;

// Return list with 1, 2, 3
fn init_test_list() -> LinkedList<i32> {
    let mut list = LinkedList::<i32>::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    list.push(6);

    list
}

#[test]
fn push_pop_peek() {
    let mut list = init_test_list();

    assert_eq!(list.peek(), Some(1));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.peek(), Some(2));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.peek(), Some(3));
    assert_eq!(list.pop(), Some(3));
}

#[test]
fn into_iter() {
    let mut iter = init_test_list().into_iter();

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
}

#[test]
fn iter() {
    let list = init_test_list();
    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));

    assert_eq!(list.peek(), Some(1));
}

#[test]
fn iter_mut() {
    let mut list = init_test_list();

    for val in list.iter_mut() {
        *val *= 2;
    }

    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(6));
}

#[test]
fn get() {
    let list = init_test_list();

    assert_eq!(list.get(4), Some(5));
    assert_eq!(list.get(1), Some(2));
    assert_eq!(list.get(0), Some(1));
    assert_eq!(list.get(99), None);
}

#[test]
fn drop() {
    let mut list = init_test_list();

    assert_eq!(list.drop(1), Some(2));
    assert_eq!(list.drop(1), Some(3));
    assert_eq!(list.drop(1), Some(4));
    assert_eq!(list.drop(4), None);
    assert_eq!(list.drop(0), Some(1));
}


fn main() {
}