/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        if index < 0 { return None }
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn reverse(&mut self){
		// TODO
        if let (Some(mut l), Some(mut r)) = (self.start, self.end) {
            loop {
                if l.as_ptr() == r.as_ptr() {
                    return
                }
                let nextl = unsafe { (*l.as_ptr()).next };
                let prevr = unsafe { (*r.as_ptr()).prev };
                // 交换节点
                unsafe { self.swap(&mut (*l.as_ptr()), &mut (*r.as_ptr())) };
                if unsafe {(*r.as_ptr()).next.unwrap().as_ptr() == l.as_ptr()} {
                    return
                }
                // 更新 l 和 r
                l = nextl.unwrap();
                r = prevr.unwrap();

            }
        }
	}
    pub unsafe fn swap(&mut self, ref_l: &mut Node<T>, ref_r: &mut Node<T>) { // Safety: must ensure the nodes are in the linked list
        if ref_l as *mut Node<T> == ref_r as *mut Node<T> {
            return
        }

        let prevl = ref_l.prev;

        unsafe { self.extract(ref_l) };
        unsafe { self.insert(ref_r, ref_l) };

        unsafe { self.extract(ref_r) };
        if let Some(prevl) = prevl {
            let ref_prevl = unsafe { &mut *prevl.as_ptr() }; 
            unsafe { self.insert(ref_prevl, ref_r) };
        } else {
            ref_r.next = self.start;
            ref_r.prev = None;
            unsafe { (*self.start.unwrap().as_ptr()).prev = Some(NonNull::new_unchecked(ref_r)) };
            self.start = Some(unsafe { NonNull::new_unchecked(ref_r)});
        } 
    }
    pub unsafe fn extract(&mut self, node: &mut Node<T>) { // Safety: must ensure the node is in the linked list
        if let Some(prev) = node.prev {
            unsafe { (*prev.as_ptr()).next = node.next }
        } else {
            self.start = node.next;
        }
        if let Some(next) = node.next {
            unsafe { (*next.as_ptr()).prev = node.prev }
        } else {
            self.end = node.prev;
        }
    }
    pub unsafe fn insert(&mut self, target_node: &mut Node<T>, node: &mut Node<T>) { // Safety: must ensure the nodes are in the linked list and not the same node
        node.next = target_node.next;
        node.prev = unsafe { Some(NonNull::new_unchecked(target_node)) };
        if let Some(target_next) = target_node.next {
            unsafe { (*target_next.as_ptr()).prev = Some(NonNull::new_unchecked(node)) }
        } else {
            self.end = unsafe { Some(NonNull::new_unchecked(node)) };
        }
        target_node.next = unsafe { Some(NonNull::new_unchecked(node)) };
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}