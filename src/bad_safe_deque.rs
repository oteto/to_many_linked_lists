use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct BadSafeDeque<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> BadSafeDeque<T> {
    pub fn new() -> Self {
        BadSafeDeque {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        // キューの先頭の要素を剥がす
        match self.head.take() {
            // 先頭の要素がある場合は、その要素の prev に追加する要素をセットし、
            // 追加する要素の next に先頭の要素をセットする
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            // キューが空の場合は、head, tail の両方に要素をセットする
            None => {
                self.tail = Some(Rc::clone(&new_head));
                self.head = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(Rc::clone(&new_tail));
                self.head = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // キューの先頭の要素を剥がす
        self.head.take().map(|old_head| {
            // 取り除いた要素から次の要素を剥がす
            match old_head.borrow_mut().next.take() {
                // 次の要素がある場合は、その要素から prev すなわちキューの先頭の要素を剥がしたものをキューの先頭にセットする
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                // 次の要素がない場合（キューの要素が１つだった場合）は、キューの最後の要素も剥がす
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
}

impl<T> Drop for BadSafeDeque<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test_bad_safe_deque {
    use super::BadSafeDeque;

    #[test]
    fn basics() {
        let mut deque = BadSafeDeque::new();
        assert_eq!(deque.pop_front(), None);
        assert_eq!(deque.pop_back(), None);

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_back(), Some(1));

        deque.push_back(4);
        deque.push_back(5);

        assert_eq!(deque.pop_back(), Some(5));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_back(), Some(4));
        assert_eq!(deque.pop_front(), None);
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut deque = BadSafeDeque::new();
        assert!(deque.peek_front().is_none());
        assert!(deque.peek_front_mut().is_none());
        assert!(deque.peek_back().is_none());
        assert!(deque.peek_back_mut().is_none());

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        assert_eq!(*deque.peek_front().unwrap(), 3);
        assert_eq!(*deque.peek_back().unwrap(), 1);

        if let Some(mut node) = deque.peek_front_mut() {
            *node = 4;
        };
        assert_eq!(*deque.peek_front().unwrap(), 4);

        if let Some(mut node) = deque.peek_back_mut() {
            *node = 5;
        };
        assert_eq!(*deque.peek_back().unwrap(), 5);
    }
}
