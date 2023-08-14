mod funcs {
    pub fn mod_2() {
        println!("Module: {}", file!())
    }
}

// The pub keyword may also be used in a use declaration to re-export an identifier from a namespace.
//
// If I hadn't include this the mod_3 function would be in another path.
pub use funcs::mod_2;
// This is the same as "pub use self::funcs::mod_2;"