
fn main() {
    println!("{}", is_palindrom("abc", "cba"));
    println!("{}", is_palindrom("abca", "cba"));
    println!("{}", is_palindrom("aaz", "zza"));
    println!("{}", is_palindrom("rac", "car"));
}

fn is_palindrom(a: &str, b: &str) -> bool {
    if a.chars().count() != b.chars().count() {
        return false;
    }

    for (i, a_char) in a.chars().enumerate() {
        let end_index = a.len() - i;
        let start_index = if end_index > 0 { end_index - 1 } else { 0 };
        let b_char = b.get(start_index..end_index).unwrap_or("");

        if *b_char != a_char.to_string() {
            return false;
        }
    }

    true
}
