
pub struct List {
    head: Link,
}

#[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Clone)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.clone(),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}