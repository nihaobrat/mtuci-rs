struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Vector { data: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Vector { data: Vec::with_capacity(capacity) }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.swap_remove(index))
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn resize(&mut self, new_size: usize)
    where
        T: Clone + Default,
    {
        self.data.resize(new_size, T::default());
    }

    fn clear(&mut self)
    where
        T: Clone,
    {
        self.data.clear();
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}

fn main() {
    let mut vector: Vector<i32> = Vector::new();

    vector.push(1);
    vector.push(2);
    vector.push(3);

    match vector.pop() {
        Some(value) => println!("Значение успешно удалено из вектора: {:?}", value),
        None => println!("Вектор очищен"),
    }

    match vector.remove(0) {
        Some(value) => println!("Значение, которое удалено из вектора: {:?}", value),
        None => println!("Значение индекса недопустимо"),
    }

    vector.resize(2);

    for item in vector.iter() {
        println!("Значение: {:?}", item);
    }

    vector.clear();
    println!("Вектор очищен: {:?}", vector.is_empty());
}
