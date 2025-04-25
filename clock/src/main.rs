use clock::Clock;

fn main() {
    println!(
        "{} {}",
        Clock::new(18, 7).to_string(),
        Clock::new(-54, -11513).to_string()
    )
}
