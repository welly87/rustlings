// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.

// TODO: always think of allocation when you see object, string etc. that's why avoid Json etc

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}
