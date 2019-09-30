use crate::hashing::Hashing;

const BUCKET_SIZE: usize = 4;

pub struct MyHash<K, T>
{
    inner: Vec<Option<(K, T)>>
}

impl<K, T> MyHash<K, T>
    where K: Hashing + Clone + PartialEq,
          T: Clone
{
    fn calc_index(&self, key: &K) -> usize
    {
        key.hashing() % (self.inner.len() / BUCKET_SIZE) * BUCKET_SIZE
    }

    pub fn new() -> Self
    {
        let mut new_vec = Vec::new();
        for _ in 0..(16 * BUCKET_SIZE)
        {
            new_vec.push(None)
        }
        Self
        {
            inner: new_vec
        }
    }

    /// Insert to hashmap
    /// ```
    ///     # use hashmap::my_hash::MyHash;
    ///     let mut bla = MyHash::new();
    ///     bla.insert("bla".to_string(), "Blabla");
    ///     assert!(bla.get("bla".to_string()), Some("Blabla"));
    /// ```
    pub fn insert(&mut self, key: K, value: T) -> Result<(), &str>
    {
        let index = self.calc_index(&key);

        for i in index..index+BUCKET_SIZE
        {
            match &self.inner[i]
            {
                Some((k, _)) => {
                    if k == &key {
                        self.inner[i] = Some((key, value));
                        return Ok(());
                    }
                },
                None => {
                    self.inner[i] = Some((key, value));
                    return Ok(());
                },
            }
        }
        Err("No space left")
    }

    pub fn get(&self, key: K) -> Option<&T>
    {
        let index = self.calc_index(&key);

        for i in index..index+BUCKET_SIZE
        {
            match &self.inner[i]
            {
                Some((k, v)) => {
                    if k == &key {
                        return Some(v);
                    }
                },
                None => return None,
            }
        }
        None
    }

    // Removes a key from the map, returning the value at
    // the key if the key was previously in the map.
    pub fn remove(&mut self, key: K) -> Option<T>
    {
        let index = self.calc_index(&key);
        let inner_vec = &self.inner.to_vec(); // Had to add Clone trait to K :-(
        for i in index..index+BUCKET_SIZE
        {
            match &inner_vec[i]
            {
                Some((k, v)) => {
                    if k == &key {
                        self.inner[i] = None;
                        return Some(v.clone())
                    }
                },
                None => return None,
            }
        }
        None
    }

    // Clears the map, removing all key-value pairs.
    // Keeps the allocated memory for reuse.
    pub fn clear(&mut self)
    {
        self.inner.clear()
    }

    // Returns the number of elements in the map.
    pub fn len(&self) -> usize
    {
        let mut count = 0;
        for i in 0..self.inner.len()
        {
            if self.inner[i].is_some() {
                count += 1;
            }
        }
        count
    }
}

struct MyHashIter<K, T>
{
    count: usize,
    inner: Vec<Option<(K, T)>>
}

impl<K, T> MyHashIter<K, T>
{
     fn new(inner_vec: Vec<Option<(K, T)>>) -> Self
    {
        Self { count: 0, inner: inner_vec }
    }
}

impl<K, T> Iterator for MyHashIter<K, T>

{
    type Item = (K, T);
    fn next(&mut self) -> Option<Self::Item>
    {
        let count = self.count;
        for i in count..self.inner.len()
        {
            if self.inner[i].is_some()
            {
                self.count += 1;
                return self.inner[i] // dunno what to do here
            }
            self.count += 1;
        }
        None
    }
}
