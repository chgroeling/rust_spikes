use std::env;

fn main() {
    let cwd = env::current_dir();
    if let Ok(i) = &cwd {
        println!("current working directory {:?}", i);
    }
}
