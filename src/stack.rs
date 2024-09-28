#![allow(dead_code, unused_variables)]

use std::{borrow::Borrow, collections::HashMap};
type NodePointer<T> = Box<Node<T>>;

// ------------------------------ Node ------------------------------
#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NodePointer<T>>
}

impl <T> Node<T> {
    fn new(val: T, next: Option<NodePointer<T>>) -> Self {
        Node { val, next }
    }
}

// ------------------------------ Stack ------------------------------
#[derive(Debug)]
pub struct Stack<T> {
    head: Option<NodePointer<T>>,
    pub size: u32
}

impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None, size: 0 }
    }

    pub fn push(&mut self, val: T) {
        let new_node = Box::new(Node::new(val, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head_node| {
            self.head = head_node.next;
            self.size -= 1;
            head_node.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head_node| {
            &head_node.val
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { current_node: self.head.as_ref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { current_node: self.head.as_mut() }
    }
}

// ------------------------------ Stack ------------------------------
pub struct IntoIter<T> (Stack<T>);

impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    current_node: Option<&'a NodePointer<T>>
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_node.take().map(|node| {
            self.current_node = node.next.as_ref();
            &node.val
        })
    }
}

pub struct IterMut<'a, T> {
    current_node: Option<&'a mut NodePointer<T>>
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_node.take().map(|node| {
            self.current_node = node.next.as_mut();
            &mut node.val
        })
    }
}

// pushed 1-6 on the stack in that exact order
fn init_stack() -> Stack<i32> {
    let mut stack = Stack::<i32>::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);
    stack.push(6);

    stack
}

#[test]
fn push() {
    let mut stack = init_stack();

    assert_eq!(stack.pop(), Some(6));
    assert_eq!(stack.pop(), Some(5));
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[test]
fn peek() {
    let mut stack = init_stack();

    assert_eq!(stack.peek(), Some(&6));
    stack.pop();
    stack.pop();
    assert_eq!(stack.peek(), Some(&4));
    stack.push(99);
    assert_eq!(stack.peek(), Some(&99));
}

#[test]
fn is_empty() {
    let mut stack = Stack::<i32>::new();

    assert_eq!(stack.is_empty(), true);
    stack.push(5);
    assert_eq!(stack.is_empty(), false);
    stack.pop();
    assert_eq!(stack.is_empty(), true);
}

#[test]
fn into_iter() {
    let mut iter = init_stack().into_iter();

    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let stack = init_stack();
    let mut iter = stack.iter();

    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut() {
    let mut stack = init_stack();
    let mut iter = stack.iter_mut();

    assert_eq!(iter.next(), Some(&mut 6));
    assert_eq!(iter.next(), Some(&mut 5));
    assert_eq!(iter.next(), Some(&mut 4));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), None);
}

// ------------------------------ Application ------------------------------
fn brackets_are_valid(brackets: &str) -> bool {
    let mut stack = Stack::new();

    let closing_brackets = [']', ')', '}'];
    let mut bracket_pairs = HashMap::new();
    bracket_pairs.insert('(', ')');
    bracket_pairs.insert('[', ']');
    bracket_pairs.insert('{', '}');

    for bracket in brackets.chars() {
        // Checking if bracket is a closing bracket
        if closing_brackets.contains(&bracket) {
            // Checking if the bracket is the closing pair for the bracket that's the head
            if let Some(head) = stack.peek() {
                let head_closing_pair = bracket_pairs.get(head);
                if head_closing_pair == Some(bracket.borrow()) {
                    stack.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            stack.push(bracket);
        }
    }

    stack.is_empty()
}

#[test]
fn check_brackets_are_valid() {
    assert_eq!(brackets_are_valid("[({})]"), true);
    assert_eq!(brackets_are_valid("[(})]"), false);
    assert_eq!(brackets_are_valid("({])"), false);
    assert_eq!(brackets_are_valid("{}"), true);
    assert_eq!(brackets_are_valid(""), true);
}