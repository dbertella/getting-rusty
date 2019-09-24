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
    pub fn new() -> Self
    {
        Self { inner: vec![None; 16 * BUCKET_SIZE] }
    }

    fn calc_index(&self, key: &K) -> usize
    {
        key.hashing() % (self.inner.len() / BUCKET_SIZE) * BUCKET_SIZE
    }
    
    /// Insert to hashmap
    /// ```
    ///     # use hashmap::my_hash::MyHash;
    ///     let mut bla = MyHash::new();
    ///     bla.insert("bla".to_string(), "Blabla");
    ///     assert!(bla.get("bla".to_string()), Some("Blabla"));
    /// ```
    pub fn insert(&mut self, key: K, value: T)
    {
        let index = self.calc_index(&key);

        for i in index..index+BUCKET_SIZE
        {
            match &self.inner[i]
            {
                Some((k, _)) => {
                    if k == &key {
                        self.inner[i] = Some((key, value));
                        return;
                    }
                },
                None => {
                    self.inner[i] = Some((key, value));
                    return;
                },
            }
        }
        panic!("No space left");
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
}
