#[cfg(test)]
mod list_tests {
    use core::ptr;
    use crate::list::List;

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
            (0..10).for_each(|i| list.push(&mut ptrs[i] as *mut usize));
        }
        assert!(!list.is_empty());
        for i in 0..10 {
            assert_eq!(list.pop().unwrap(), &mut ptrs[9 - i] as *mut usize);
        }
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        let mut ptrs = [0usize; 10];
        unsafe {
            (0..10).for_each(|i| list.push(&mut ptrs[i] as *mut usize));
        }
        let mut iter = list.iter();
        for i in 0..=8 {
            assert_eq!(
                unsafe { *(*iter.next().unwrap() as *mut usize) },
                ptrs[8 - i]
            );
        }
        assert_eq!(
            unsafe { *iter.next().unwrap() as *mut usize },
            ptr::null_mut()
        );
        assert_eq!(iter.next(), None);
    }
}


#[cfg(test)]
mod buddy_tests {
    use core::{alloc::Layout, mem::size_of};
    use crate::buddy::Heap;

    #[test]
    fn test_empty_heap() {
        let mut heap = Heap::<10>::new();
        assert_eq!(heap.total(), 0);
        assert_eq!(heap.allocated(), 0);
        assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());
    }

    #[test]
    fn test_add_range() {
        let mut heap = Heap::<32>::new();
        assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());
    
        let space: [usize; 100] = [0; 100];
        unsafe {
            heap.add_range(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
        }
        let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap());
        assert!(addr.is_ok());
    }
    
    #[test]
    fn test_add_size() {
        let mut heap = Heap::<32>::new();
        assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());
    
        let space: [usize; 100] = [0; 100];
        unsafe {
            heap.add_size(space.as_ptr() as usize, 100);
        }
        let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap());
        assert!(addr.is_ok());
    }

    #[test]
    fn test_alloc_dealloc() {
        let mut heap = Heap::<32>::new();
        let space: [usize; 256] = [0; 256];
        unsafe {
            heap.add_size(space.as_ptr() as usize, 256 * size_of::<usize>());
        }
        for _ in 0..256 {
            let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap()).unwrap();
            heap.dealloc(addr, Layout::from_size_align(1, 1).unwrap());
        }
        heap.alloc(Layout::from_size_align(96, 1).unwrap()).unwrap();
    }

    #[test]
    fn test_alloc_large() {
        let mut heap = Heap::<32>::new();
        let space: [usize; 256] = [0; 256];
        unsafe {
            heap.add_size(space.as_ptr() as usize, 256 * size_of::<usize>());
        }
        assert!(heap.alloc(Layout::from_size_align(4096, 1).unwrap()).is_err());
    }
}
#[cfg(test)]
mod alloc_tests {
    use core::alloc::GlobalAlloc;
    use crate::Allocator;

    static ALLOCATOR: Allocator<32> = Allocator::new();

    #[test]
    fn test_alloc_dealloc() {
       let space: [u8; 1500] = [0; 1500];
       unsafe {
           ALLOCATOR.0.lock().add_size(space.as_ptr() as usize, 2048);
       }
       let mem1 = unsafe {ALLOCATOR.alloc(core::alloc::Layout::from_size_align(512, 1).unwrap())};
       assert!(!mem1.is_null());
       let mem2 = unsafe {ALLOCATOR.alloc(core::alloc::Layout::from_size_align(512, 1).unwrap())};
       assert!(!mem2.is_null());
       let mem3 = unsafe {ALLOCATOR.alloc(core::alloc::Layout::from_size_align(1024, 1).unwrap())};
       assert!(mem3.is_null());
       unsafe {
           ALLOCATOR.dealloc(mem1, core::alloc::Layout::from_size_align(512, 1).unwrap());
           ALLOCATOR.dealloc(mem2, core::alloc::Layout::from_size_align(512, 1).unwrap());
       }
       let mem4 = unsafe {ALLOCATOR.alloc(core::alloc::Layout::from_size_align(1024, 1).unwrap())};
       assert!(!mem4.is_null());
    }
}