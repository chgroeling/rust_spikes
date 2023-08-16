pub mod funcs;

// The pub keyword may also be used in a use declaration to re-export an identifier from a namespace.
//
// If I hadn't include this the mod_3 function would be in another path.
pub use self::funcs::mod_5; // self is redundant here

