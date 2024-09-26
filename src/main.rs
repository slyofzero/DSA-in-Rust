mod linked_list {
    use std::{cell::RefCell, fmt::Display, rc::Rc};

    type NodePointer<T> = Option<Rc<RefCell<Node<T>>>>;

    #[derive(Debug)]
    pub struct Node<T> {
        val: T,
        next: NodePointer<T>
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

    // ------------------------------ Node
    #[derive(Debug)]
    pub struct LinkedList<T> {
        head: NodePointer<T>,
        tail: NodePointer<T>,
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

    impl <T: Clone> Iterator for LinkedList<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(head_node_rc) = self.head.clone() {
                let new_head = head_node_rc.borrow().next.clone();
                self.head = new_head;
                Some(head_node_rc.borrow().val.clone())
            } else {
                None
            }
        }
    }

    impl <T> LinkedList<T> {
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
    }
}

use linked_list::LinkedList;

#[test]
fn calling_next() {
    let mut list = LinkedList::<i32>::new();

    list.push(5);
    list.push(7);
    list.push(1);

    // list.insert_at(99, 2);

    assert_eq!(list.next(), Some(5));
    assert_eq!(list.next(), Some(7));
    assert_eq!(list.next(), Some(1));
}

fn main() {
    let mut list = LinkedList::<i32>::new();

    list.push(5);
    list.push(7);
    list.push(1);

    list.insert_at(99, 2);

    for num in list {
        println!("{}", num);
    }

    // println!("{}", list);
}