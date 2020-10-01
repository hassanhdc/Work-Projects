use std::mem;

struct List {
    head: Link,
}
struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    More(Box<Node>),
    Empty,
}

impl List {
    fn new() -> Self {
        List { head: Link::Empty }
    }

    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node)
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur {
            cur = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

fn main() {}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn basic() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        //
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        //
        list.push(4);
        list.push(5);
        //
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), None);
    }
}
