use std::slice::Iter;
use crate::hashing::Hashing;

const BUCKET_SIZE: usize = 4;

#[derive(Debug)]
pub struct MyHash<K, T>
{
    inner: Vec<Option<(K, T)>>
}
impl<K, T> MyHash<K, T>
where
    K: Hashing + PartialEq,
    T: Clone,
{
    pub fn calc_index(&self, key: &K) -> usize {
        key.hashing() % (self.inner.len() / BUCKET_SIZE) * BUCKET_SIZE
    }

    pub fn new() -> Self {
        let mut new_vec = Vec::new();
        for _ in 0..(16 * BUCKET_SIZE) {
            new_vec.push(None)
        }
        Self { inner: new_vec }
    }

    /// Insert to hashmap
    /// ```
    ///     # use hashmap::my_hash::MyHash;
    ///     let mut bla = MyHash::new();
    ///     bla.insert("bla".to_string(), "Blabla");
    ///     assert!(bla.get("bla".to_string()), Some("Blabla"));
    /// ```
    pub fn insert(&mut self, key: K, value: T) -> Result<(), &str> {
        let index = self.calc_index(&key);
        for i in index..index + BUCKET_SIZE {
            match &self.inner[i] {
                Some((k, _)) => {
                    if k == &key {
                        self.inner[i] = Some((key, value));
                        return Ok(());
                    }
                }
                None => {
                    self.inner[i] = Some((key, value));
                    return Ok(());
                }
            }
        }
        Err("No space left")
    }

    pub fn get(&self, key: K) -> Option<&T> {
        let index = self.calc_index(&key);

        for i in index..index + BUCKET_SIZE {
            match &self.inner[i] {
                Some((k, v)) => {
                    if k == &key {
                        return Some(v);
                    }
                }
                None => return None,
            }
        }
        None
    }

    // Removes a key from the map, returning the value at
    // the key if the key was previously in the map.
    pub fn remove(&mut self, key: K) -> Option<T> {
        let index = self.calc_index(&key);

        for i in index..index + BUCKET_SIZE {
            match &self.inner[i] {
                Some((k, _)) => {
                    if k == &key {
                        if let Some((_, value)) = self.inner.remove(index) {
                            self.inner.insert(index + BUCKET_SIZE, None);
                            return Some(value);
                        }
                    }
                }
                None => return None,
            }
        }
        None
    }

    // Clears the map, removing all key-value pairs.
    // Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    // Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        let mut count = 0;
        for i in 0..self.inner.len() {
            if self.inner[i].is_some() {
                count += 1;
            }
        }
        count
    }

    pub fn keys(&self) -> MyHashKeyIter<'_, K, T> {
        MyHashKeyIter::new(self.inner.iter())
    }

    pub fn values(&self) -> MyHashValueIter<'_, K, T> {
        MyHashValueIter::new(self.inner.iter())
    }

    pub fn pairs(&self) -> MyHashItemIter<'_, K, T>
    {
        MyHashItemIter::new(self.inner.iter())
    }
}

pub struct MyHashKeyIter<'a, K, T> {
    iter: MyHashItemIter<'a, K, T>,
}

impl<'a, K, T> MyHashKeyIter<'a, K, T> {
    fn new(iter: Iter<'a, Option<(K, T)>>) -> Self {
        Self {
            iter: MyHashItemIter::new(iter),
        }
    }
}

impl<'a, K, T> Iterator for MyHashKeyIter<'a, K, T> {
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }
}

pub struct MyHashValueIter<'a, K, T> {
    iter: MyHashItemIter<'a, K, T>,
}

impl<'a, K, T> MyHashValueIter<'a, K, T> {
    fn new(iter: Iter<'a, Option<(K, T)>>) -> Self {
        Self {
            iter: MyHashItemIter::new(iter),
        }
    }
}

impl<'a, K, T> Iterator for MyHashValueIter<'a, K, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((_, v)) => Some(v),
            None => None,
        }
    }
}

pub struct MyHashItemIter<'a, K, T> {
    iter: Iter<'a, Option<(K, T)>>,
}

impl<'a, K, T> MyHashItemIter<'a, K, T> {
    fn new(iter: Iter<'a, Option<(K, T)>>) -> Self {
        Self { iter }
    }
}

impl<'a, K, T> Iterator for MyHashItemIter<'a, K, T> {
    type Item = &'a (K, T);
    fn next(&mut self) -> Option<Self::Item>
    {
        for pair in self.iter.by_ref()
        {
            match pair
            {
                Some(pair) => return Some(pair),
                None => continue,
            }
        }
        None
    }
}
