use std::collections::LinkedList;


pub struct HashTable<K, V>
{
    elements: Vec<LinkedList<(K, V)>>,
    count: usize,
}

impl<K: Hashable + std::cmp::PartialEq, V> Default for HashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl<K: Hashable + std::cmp::PartialEq, V> HashTable<K, V> {
    pub fn new() -> HashTable<K, V> {
        let initial_capacity = 3000;
        let mut elements = Vec::with_capacity(initial_capacity);
        for _ in 0..initial_capacity {
            elements.push(LinkedList::new())
        }
        HashTable{elements, count: 0}
    }

    pub fn insert(&mut self, key: K, value: V){
        if self.count >= self.elements.len() * 3 / 4 {
            self.resize();
        }
        let index = key.hash() & self.elements.len();
        self.elements[index].push_back((key, value));
        self.count += 1;
    }

    pub fn search(&self, key: K) ->Option<&V>{
        let index = key.hash() & self.elements.len();
        self.elements[index].iter().find(|(k, _)| *k==key).map(|(_, v)|v)
    }

    fn resize(&mut self){
        let new_size = self.elements.len() * 2;
        let mut new_elements = Vec::with_capacity(new_size);
        for _ in 0..new_size{
            new_elements.push(LinkedList::new());
        }
        for old_list in self.elements.drain(..) {
            for (key, value) in old_list {
                let new_index = key.hash() % new_size;
                new_elements[new_index].push_back((key, value));
            }
        }

        self.elements = new_elements;
    }
}

#[cfg(test)]
mod test{
    use super::{HashTable, Hashable};
    #[derive(Debug, PartialEq, Eq)]
    struct TestKey(usize);

    impl Hashable for TestKey {
        fn hash(&self) -> usize {
            self.0
        }
    }
    #[test]
    fn test_insert_and_search() {
        let mut hash_table = HashTable::new();
        let key = TestKey(1);
        let value = TestKey(10);

        hash_table.insert(key, value);
        let result = hash_table.search(TestKey(1));

        assert_eq!(result, Some(&TestKey(10)));
    }

    

}