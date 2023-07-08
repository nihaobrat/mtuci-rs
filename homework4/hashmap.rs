use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Clone + Hash + PartialEq,
    V: Clone,
{
    const INITIAL_CAPACITY: usize = 16;
    const LOAD_FACTOR_THRESHOLD: f64 = 0.75;

    fn new() -> Self {
        let buckets = vec![Vec::new(); Self::INITIAL_CAPACITY];
        HashMap {
            buckets,
            size: 0,
        }
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn get_bucket_index(&self, key: &K) -> usize {
        let hash = self.hash(key);
        (hash as usize) % self.buckets.len()
    }

    fn insert(&mut self, key: K, value: V) {
        if (self.size as f64) / (self.buckets.len() as f64) >= Self::LOAD_FACTOR_THRESHOLD {
            self.resize();
        }

        let bucket_index = self.get_bucket_index(&key);
        let bucket = &mut self.buckets[bucket_index];

        for &(ref existing_key, _) in bucket.iter() {
            if existing_key == &key {
                // Если ключ уже существует, обновляем значение и возвращаемся.
                for i in 0..bucket.len() {
                    if bucket[i].0 == key {
                        bucket[i] = (key.clone(), value.clone());
                        return;
                    }
                }
            }
        }

        bucket.push((key, value));
        self.size += 1;
    }

    fn get(&self, key: &K) -> Option<&V> {
        let bucket_index = self.get_bucket_index(key);
        let bucket = &self.buckets[bucket_index];

        for &(ref existing_key, ref value) in bucket.iter() {
            if existing_key == key {
                return Some(value);
            }
        }

        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.get_bucket_index(key);
        let bucket = &mut self.buckets[bucket_index];

        for i in 0..bucket.len() {
            if bucket[i].0 == *key {
                let (_, value) = bucket.remove(i);
                self.size -= 1;
                return Some(value);
            }
        }

        None
    }

    fn resize(&mut self) {
        let new_capacity = self.buckets.len() * 2;
        let mut new_buckets = vec![Vec::new(); new_capacity];

        for bucket in &self.buckets {
            for &(ref key, ref value) in bucket {
                let new_bucket_index = self.get_hashed_bucket_index(key, &new_buckets);
                new_buckets[new_bucket_index].push((key.clone(), value.clone()));
            }
        }

        self.buckets = new_buckets;
    }

    fn get_hashed_bucket_index(&self, key: &K, buckets: &[Vec<(K, V)>]) -> usize {
        let hash = self.hash(key);
        (hash as usize) % buckets.len()
    }
}

fn main() {
    let mut hashmap: HashMap<i32, String> = HashMap::new();
    hashmap.insert(1, "Значение 1".to_string());
    hashmap.insert(2, "Значение 2".to_string());
    hashmap.insert(3, "Значение 3".to_string());

    println!("{:?}", hashmap.get(&1));
    println!("{:?}", hashmap.get(&2));
    println!("{:?}", hashmap.get(&3));

    hashmap.remove(&2);

    println!("{:?}", hashmap.get(&2));
}
