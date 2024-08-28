use std::env;
pub fn setup() {
    println!(
        "访问Current directory: {}",
        env::current_dir().unwrap().display()
    );
}
