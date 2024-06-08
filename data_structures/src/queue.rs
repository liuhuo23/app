use crate::linked_list::LinkedList;
#[derive(Debug)]
pub struct Queue<T>{
    elements: LinkedList<T>
}

impl<T> Queue<T>{
    pub fn new()->Queue<T>{
        Queue{
            elements: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T){
        self.elements.insert_at_tail(value)
    }

    pub fn dequeue(&mut self) ->Option<T>{
        self.elements.delete_head()
    }

    pub fn peek_front(&self) -> Option<&T>{
        self.elements.get(0)

    }
    pub fn len(&self) -> usize {
        self.elements.length as usize
    }

    pub fn is_empty(&self) -> bool {
        self.elements.length == 0
    }
}

impl<T> Default for Queue<T>{
    fn default() -> Self {
        Queue::new()
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn test_queue(){
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(64);
        assert!(!queue.is_empty(), "Queue should not be empty after enqueue");
    }

    #[test]
    fn test_dequeue(){
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(32);
        queue.enqueue(53);
        queue.enqueue(23);
        let back = queue.dequeue();
        assert_eq!(
            back,
            Some(32),
            "Dequeue should return the first element"
        )
    }

    #[test]
    fn test_peek_front(){
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(12);
        queue.enqueue(14);
        let fron = queue.peek_front();
        assert_eq!(
            fron,
            Some(&12),
            "Peek should return a reference to the first element"
        )
    }
}