use std::rc::Rc;

pub struct PersistentStack<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> PersistentStack<T> {
    pub fn new() -> Self {
        PersistentStack { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        PersistentStack {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.as_ref().map(|node| Rc::clone(node)),
            })),
        }
    }

    pub fn tail(&self) -> Self {
        PersistentStack {
            head: self
                .head
                .as_ref()
                .and_then(|node| node.next.as_ref().map(|node| Rc::clone(&node))),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> PersistentStack<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            // next: self.head.as_ref().map(|node| &**node),
            // next: self.head.as_ref().map::<&Node<T>, _>(|node| &node),
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

impl<T> Drop for PersistentStack<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test_persistent_stack {
    use super::PersistentStack;

    #[test]
    fn basics() {
        let stack = PersistentStack::new();
        assert_eq!(stack.head(), None);

        let stack = stack.prepend(1).prepend(2).prepend(3);
        assert_eq!(stack.head(), Some(&3));

        let stack = stack.tail();
        assert_eq!(stack.head(), Some(&2));

        let stack = stack.tail();
        assert_eq!(stack.head(), Some(&1));

        let stack = stack.tail();
        assert_eq!(stack.head(), None);
    }

    #[test]
    fn iter() {
        let stack = PersistentStack::new().prepend(1).prepend(2).prepend(3);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
