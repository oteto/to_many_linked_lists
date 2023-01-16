pub struct OkStack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> OkStack<T> {
    pub fn new() -> Self {
        OkStack { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for OkStack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(OkStack<T>);

impl<T> OkStack<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> OkStack<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test_ok_stack {
    use super::OkStack;

    #[test]
    fn basics() {
        let mut stack = OkStack::new();

        assert_eq!(stack.pop(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);
        stack.push(5);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek() {
        let mut stack = OkStack::new();
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        if let Some(v) = stack.peek_mut() {
            *v = 42;
        }
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut stack = OkStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut stack = OkStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
