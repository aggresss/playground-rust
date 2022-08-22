use std::{marker::PhantomData, ptr::NonNull};

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
            prev: None,
        }
    }
}

pub struct LinkedList<T> {
    length: u32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }

    pub fn insert_at_head(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = self.head;
        node.prev = None;
        let node_ptr = Some(unsafe {
            NonNull::new_unchecked(Box::into_raw(node));
        });

    }
}
