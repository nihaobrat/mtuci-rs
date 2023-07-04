const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

fn main() {
    let result = find_term(SEARCH_TERM, QUOTE);
    println!("{}", result);
}

fn find_term(search_term: &str, quote: &str) -> String {
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            return format!("{}: {}", i + 1, line.trim());
        }
    }
    String::new()
}
