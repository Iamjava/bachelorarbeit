use tokio::sync::RwLock;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::collections::hash_map::Entry;
use std::hash::Hash;

struct BufferManager<U,V> {
    buffer: RwLock<HashMap<U, V>>,
    lru_list: RwLock<LinkedList<U>>,
    capacity: usize,
}

impl<U : Eq+PartialEq+Hash + Clone,V: Copy> BufferManager<U,V> {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: RwLock::new(HashMap::new()),
            lru_list: RwLock::new(LinkedList::new()),
            capacity,
        }
    }

    async fn get(&self, key: &U) -> Option<V> {
        let  buffers = self.buffer.read().await;
        let mut lru_list = self.lru_list.write().await;
        let mut ret = None;

        if let Some(value) = buffers.get(key) {
            // Move the accessed key to the back of the LRU list
            let key = key.to_owned();
            let val = value.clone();
            lru_list.push_back(key);
            ret = Some(val);
        }
        return ret;
    }

    async fn insert(&self, key: U, value: V) {
        let mut buffers = self.buffer.write().await;
        let mut lru_list = self.lru_list.write().await;


        match buffers.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                // Update the value and move the key to the back of the LRU list
                *entry.get_mut() = value;
                lru_list.push_back(key);
            }
            Entry::Vacant(entry) => {
                // Insert the new key and value and add it to the back of the LRU list
                entry.insert(value);
                lru_list.push_back(key);

                // Evict the least recently used key if the capacity is reached
                if buffers.len() > self.capacity {
                    if let Some(key) = lru_list.pop_front() {
                        buffers.remove(&key);
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_buf_man() {
        let bm = BufferManager::new(2);
        let _ = bm.insert("Hello","world").await;
        assert_eq!(bm.buffer.read().await.len(),1);
        let _ = bm.insert("Hello2","world").await;
        let _ = bm.insert("Hello3","world").await;
        assert_eq!(bm.buffer.read().await.len(),2);
        bm.get(&"Hello3").await;
        assert_eq!(bm.lru_list.read().await.back().unwrap(),&"Hello3");

        bm.get(&"Hello2").await;
        assert_eq!(bm.lru_list.read().await.back().unwrap(),&"Hello2");
    }
}
