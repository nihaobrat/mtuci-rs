use rand::Rng;

fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let byte = rng.sample(rand::distributions::Alphanumeric);
            char::from(byte)
        })
        .collect();

    password
}

fn main() {
    let password_length = 12;
    let password = generate_password(password_length);
    println!("{}", password);
}

