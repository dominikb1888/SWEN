use std::mem;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {      
        let new_node = Box::new( Node {
            elem: elem,
            next: self.head.take()
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
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T: Ord> List<T> {
    pub fn vec_sort(&mut self) {
        if self.head.is_none() { return; }
        
        let mut v: Vec<T> = std::mem::take(self).into_iter().collect();
        v.sort();
        
        for item in v.into_iter().rev() {
            self.push(item);
        }
    }

    pub fn merge_sort(&mut self) {
        self.head = Self::merge_sort_rec(self.head.take());
    }

    fn merge_sort_rec(head: Link<T>) -> Link<T> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let (l1, l2) = Self::split(head);
        let l1 = Self::merge_sort_rec(l1);
        let l2 = Self::merge_sort_rec(l2);
        Self::merge(l1, l2)
    }

    fn split(mut head: Link<T>) -> (Link<T>, Link<T>) {
        let mut l1 = None;
        let mut l2 = None;
        let mut toggle = true;
        while let Some(mut node) = head {
            head = node.next.take();
            if toggle {
                node.next = l1;
                l1 = Some(node);
            } else {
                node.next = l2;
                l2 = Some(node);
            }
            toggle = !toggle;
        }
        (l1, l2)
    }

    fn merge(mut l1: Link<T>, mut l2: Link<T>) -> Link<T> {
        let mut head = None;
        let mut tail = &mut head;

        while l1.is_some() && l2.is_some() {
            let take_l1 = l1.as_ref().unwrap().elem <= l2.as_ref().unwrap().elem;
            if take_l1 {
                *tail = l1;
                l1 = tail.as_mut().unwrap().next.take();
            } else {
                *tail = l2;
                l2 = tail.as_mut().unwrap().next.take();
            }
            tail = &mut tail.as_mut().unwrap().next;
        }
        *tail = if l1.is_some() { l1 } else { l2 };
        head
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
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

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_push_pop() {
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

    #[test]
    fn test_merge_sort() {
        let mut list = List::new();
        list.push(1);
        list.push(5);
        list.push(2);
        list.push(4);
        list.push(3);
        list.merge_sort();
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_vec_sort() {
        let mut list = List::new();
        list.push(1);
        list.push(5);
        list.push(2);
        list.push(4);
        list.push(3);
        list.vec_sort();
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}
