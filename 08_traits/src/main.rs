//! Rust traits explained by a simple observer pattern implementation.
//!

mod observer;
mod rc_weak;

fn main() {
    observer::example_observer();

    println!("");
   
    rc_weak::example_rc_weak();
}
