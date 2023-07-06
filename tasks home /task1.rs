const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

fn main() {
    find_term(SEARCH_TERM, QUOTE);
}

fn find_term(search_term: &str, quote: &str) -> String {
    let lines: Vec<&str> = quote.lines().collect();

    for (index, line) in lines.iter().enumerate() {
        if line.contains(search_term) {
            return format!("{}: {}", index + 1, line);
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}
