//! An intrusive singly linked list (stack) implementation without alloc crate.
//! Found here: https://cs140e.sergio.bz/assignments/2-fs/
use core::marker::PhantomData;
use core::{fmt, ptr};

#[derive(Copy, Clone)]
pub struct List {
    head: *mut usize,
}

unsafe impl Send for List {}

pub struct Node {
    prev: *mut usize,
    curr: *mut usize,
}

impl List {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub unsafe fn push(&mut self, elm: *mut usize) {
        *elm = self.head as usize;
        self.head = elm;
    }

    pub fn pop(&mut self) -> Option<*mut usize> {
        if self.is_empty() {
            None
        } else {
            let elm = self.head;
            unsafe {
                self.head = *elm as *mut usize;
            }
            Some(elm)
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            curr: self.head,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            prev: &mut self.head as *mut *mut usize as *mut usize,
            curr: self.head,
            _marker: PhantomData,
        }
    }
}

impl Node {
    pub fn value(&self) -> *mut usize {
        self.curr
    }

    pub fn pop(self) -> *mut usize {
        unsafe {
            *self.prev = *self.curr;
            self.curr
        }
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<'a> {
    curr: *mut usize,
    _marker: PhantomData<&'a List>,
}

pub struct IterMut<'a> {
    prev: *mut usize,
    curr: *mut usize,
    _marker: PhantomData<&'a mut List>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = *mut usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_null() {
            None
        } else {
            let item = self.curr;
            unsafe {
                self.curr = *item as *mut usize;
            }
            Some(item)
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_null() {
            None
        } else {
            let node = Node {
                prev: self.prev,
                curr: self.curr,
            };
            self.prev = self.curr;
            unsafe {
                self.curr = *self.curr as *mut usize;
            }
            Some(node)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list_is_empty() {
        let list: List = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_and_pop() {
        let mut list = List::new();
        let mut ptrs = [0usize; 10];
        unsafe {
            for i in 0..10 {
                list.push(&mut ptrs[i] as *mut usize);
            }
        }

        assert!(!list.is_empty());
        let mut iter = list.iter();
        for i in 0..=8 {
            assert_eq!(unsafe{ *(*iter.next().unwrap() as *mut usize) }, ptrs[8-i] );
        }
        assert_eq!(unsafe{ *iter.next().unwrap() as *mut usize }, ptr::null_mut());
        assert_eq!(iter.next(), None); 
    }
}
