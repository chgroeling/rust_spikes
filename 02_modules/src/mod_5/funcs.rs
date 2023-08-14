// crate is also used to represent the absolute path of a module, where crate refers 
// to the root of the current crate. For instance, crate::foo::bar refers to the name bar 
// inside the module foo, from anywhere else in the same crate.
use crate::test_mod; // access top_level from the crate root

pub fn mod_5() {
    println!("Module: {}", file!());
    test_mod::test_mod();
}