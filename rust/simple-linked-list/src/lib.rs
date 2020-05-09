use std::iter::FromIterator;

#[derive(Debug, Default)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Node { data, next: None }))
    }
}

#[derive(Debug, Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        self.len += 1;
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.len -= 1;
            self.head = node.next;
            node.data
        })
    }

    pub fn push_back(&mut self, _element: T) {
        let mut head = &mut self.head;
        while let Some(node) = head {
            head = &mut node.next;
        }
        *head = Node::new(_element);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut head = &mut self.head;
        while head.is_some() && head.as_ref().unwrap().next.is_some() {
            head = &mut head.as_mut().unwrap().next;
        }
        head.take().map(|val| val.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|val| &val.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let len = self.len();
        let mut head = self.head;
        let mut tail = None;
        while let Some(mut node) = head {
            head = node.next;
            node.next = tail;
            tail = Some(node);
        }
        SimpleLinkedList { head: tail, len }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in _iter {
            list.push(item);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut head = self.rev().head;
        while let Some(node) = head {
            head = node.next;
            vec.push(node.data);
        }
        vec
    }
}
