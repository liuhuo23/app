use core::fmt;
use std::{fmt::{Debug, Display, Formatter}, marker::PhantomData, ops::Deref, ptr::NonNull};

#[derive(Debug)]
pub struct Node<T>
{
    pub val: T, 
    pub next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}


impl <T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t, 
            prev: None,
            next: None
        }
    }
}



#[derive(Debug)]
pub struct LinkedList<T>{
    pub length: u32,
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    pub current: Option<NonNull<Node<T>>>,
    // Act like we own boxed nodes since we construct and leak them
    marker: PhantomData<Box<Node<T>>>,
}


impl<T> Default for LinkedList<T>{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    /// 构建函数
    pub fn new() -> Self{
        Self{
            length: 0,
            head: None,
            tail: None,
            current: None,
            marker: PhantomData,
        }
    }

    pub fn insert_at_head(&mut self, obj: T){
        // 定义一个节点
        let mut node = Box::new(Node::new(obj));
        // 将当前列表头节点的地址复制给新节点next
        node.next = self.head;
        // 初始化节点的前指针为空
        node.prev = None;
        let node_ptr = NonNull::new(Box::into_raw(node));
        // 判断链表的头指针
        match self.head {
            // 如果为None 说明链表是空的， 所以尾部也指向新的
            None => self.tail = node_ptr,
            // 如果不为空，则需要将头指针的前一个指向新的
            Some(head_ptr) => unsafe {
                (*head_ptr.as_ptr()).prev = node_ptr
            },
        }
        // 将新节点插入到头部
        self.head = node_ptr;
        // 长度加1
        self.length += 1;
        self.current = node_ptr;
    }

    pub fn insert_at_tail(&mut self, obj: T){
        // 定义一个新节点
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.tail;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe {
                (*tail_ptr.as_ptr()).next = node_ptr
            },
        }
        self.tail = node_ptr;
        self.length += 1;
        self.current = node_ptr;
    }
    pub fn insert_at_ith(&mut self, index: u32, obj: T){
        if self.length < index {
            panic!("Index out of bounds");
        }
        if index == 0 || self.head.is_none(){
            self.insert_at_head(obj);
            return;
        }
        if self.length == index {
            self.insert_at_tail(obj);
            return;
        }
        // 获取链表头 ith_node
        if let Some(mut ith_node) = self.head{
            // 一路遍历找到index说对应的指针
            for _ in 0..index {
                unsafe{
                    // 判断该指针next， 如果为空，说明这是尾指针，在这儿就不合理了， 如果不为空者获取到该位置的下一个指针
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_ptr) => ith_node = next_ptr,
                    }
                }
            }
            // 构建新的指针
            let mut node = Box::new(Node::new(obj));
            unsafe {
                // 让新节点的前指针指向该位置的前指针
                node.prev = (*ith_node.as_ptr()).prev;
                // 将新节点的下一个指向该位置的节点
                node.next = Some(ith_node);
                // 以上两行只是操作了新节点。 仍需要操作这个新节点插入后的前后两个节点
                if let Some(p) = (*ith_node.as_ptr()).prev {
                    // 获取该位置的前一个节点，然后将前一个节点的后指针指向新的节点
                    let node_ptr = NonNull::new(Box::into_raw(node));
                    print!("{:?}", (*p.as_ptr()).next);
                    (*p.as_ptr()).next = node_ptr;
                    (*ith_node.as_ptr()).prev = node_ptr;
                    self.length += 1;
                }
            }
        }
    }
    /// 删除头指针
    pub fn delete_head(&mut self) -> Option<T>{
        if self.length == 0 {
            return None;
        }
        // 拿到头指针
        self.head.map(|head_ptr|unsafe {
            // 获取原始头指针
            let old_head = Box::from_raw(head_ptr.as_ptr());
            // 判断头指针的下一个节点是否为空
            match old_head.next {
                None => self.tail = None,
                Some(mut next_ptr) => next_ptr.as_mut().prev = None,
            }
            self.head = old_head.next;
            self.length = self.length.checked_add_signed(-1).unwrap_or(0);
            old_head.val
        })
    }
    pub fn deleted_tail(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.tail.map(|tail_ptr|unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());
            match old_tail.prev {
                None => self.head = None,
                Some(mut prev_ptr) => prev_ptr.as_mut().next = None,
            }
            self.tail = old_tail.prev;
            self.length = self.length.checked_add_signed(-1).unwrap_or(0);
            old_tail.val
        })
    }

    pub fn delete_ith(&mut self, index: u32) -> Option<T>{
        if self.length < index {
            panic!("Index out of bounds");
        }
        if index == 0 || self.head.is_none(){
            return self.delete_head();
        }
        if self.length == index {
            return self.deleted_tail();
        }
        if let Some(mut ith_node) = self.head {
            for _ in 0..index {
                unsafe{
                    match (*ith_node.as_ptr()).next {
                        None => panic!(
                            "Index out of bounds"
                        ),
                        Some(next_ptr) => ith_node = next_ptr,
                    }
                }
            }
            unsafe {
                let old_ith = Box::from_raw(ith_node.as_ptr());
                if let Some(mut prev) = old_ith.prev{
                    prev.as_mut().next = old_ith.next;
                }
                if let Some(mut next) = old_ith.next {
                    next.as_mut().prev = old_ith.prev;
                }
                self.length -= 1;
                Some(old_ith.val)
            }
        }else{
            None
        }
    }
    pub fn push_back(&mut self, obj: T){
        self.insert_at_tail(obj)
    }

    pub fn get(&self, index: i32) -> Option<&T>{
        Self::get_ith_node(self.head, index).map(|ptr| unsafe {
            &(*ptr.as_ptr()).val
        })
    }
    fn get_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>>{
        match node {
            None => None,
            Some(next_ptr) => match  index {
                0 => Some(next_ptr),
                _ => Self::get_ith_node(unsafe {
                    (*next_ptr.as_ptr()).next
                }, index - 1),
            }
        }
    }

    pub fn next(&mut self) -> Option<&T>{
        match self.current {
            None => None,
            Some(current_ptr) =>{
                let current = current_ptr;
                self.current = unsafe {
                    (*current.as_ptr()).next
                };
                unsafe {
                    Some(current_ptr.as_ref())
                }
            }
        }
    }
    pub fn reset(&mut self){
        self.current = self.head;
    }
    
}


impl<T> Drop for LinkedList<T>{
    fn drop(&mut self) {
        while self.delete_head().is_none() {}
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.head {
            Some(node) =>write!(f, "{}", unsafe {
                node.as_ref()
            }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            None => write!(f, "{}", self.val),
            Some(node) => write!(f, "{}, {}", self.val, unsafe {
                node.as_ref()
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn insert_at_tail_works(){
        let mut list = LinkedList::<i32>::new();
        let sectond_value = 2;
        list.insert_at_tail(1);
        list.insert_at_tail(sectond_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, sectond_value),
            None => panic!("Expected to find {sectond_value} ast index 1")
        }
    }

    #[test]
    fn insert_at_head_works(){
        let mut list = LinkedList::<i32>::new();
        let sectond_value = 2;
        list.insert_at_head(1);
        list.insert_at_head(sectond_value);
        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, sectond_value),
            None => panic!("Expected to find {sectond_value} ast index 1")
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_tail(){
        let mut list = LinkedList::<i32>::new();
        let sectond_value = 2;
        list.insert_at_ith(0, 0);
        list.insert_at_ith(1, sectond_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, sectond_value),
            None => panic!("Expected to find {sectond_value} ast index 1")
        }
    }
    #[test]
    fn create_string_list(){
        let mut list_str = LinkedList::<String>::new();
        list_str.insert_at_tail("A".to_string());
        list_str.insert_at_tail("B".to_string());
        println!("linked list is {list_str}");
        assert_eq!(2, list_str.length);
    }
}