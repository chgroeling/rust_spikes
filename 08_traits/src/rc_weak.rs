//! Rust traits explained by a simple observer pattern implementation.
//!

use std::rc::{Rc, Weak};


struct TestObject {
    content: i32,
}
pub fn example_rc_weak() {
    println!("========================================");
    println!("{}", file!());
    println!("========================================");
    
    // not implemented
    //let obj1 = Rc::new
}
