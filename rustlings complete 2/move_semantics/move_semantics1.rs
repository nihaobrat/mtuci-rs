// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.



fn main() {
    let vec0 = Vec::new();

    let vec1 = fill_vec(vec0.clone());

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    let mut vec2 = vec1.clone();
    vec2.push(88);

    println!("{} has length {} content `{:?}`", "vec2", vec2.len(), vec2);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
