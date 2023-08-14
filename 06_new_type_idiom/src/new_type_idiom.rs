use crate::banner;
use std::ops::{Deref,Add};

struct NewTypeSimple(i32);

// The compiler is capable of providing basic implementations for some traits via the #[derive] attribute. 
// These traits can still be manually implemented if a more complex behavior is required.
//
// Comparison traits: Eq, PartialEq, Ord, PartialOrd.
// Clone, to create T from &T via a copy.
// Copy, to give a type 'copy semantics' instead of 'move semantics'.
// Hash, to compute a hash from &T.
// Default, to create an empty instance of a data type.
// Debug, to format a value using the {:?} formatter.
#[derive(Debug, Copy, Clone, PartialEq)] 
struct NewType(i32);

impl Deref for NewType {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl Add for NewType {
    type Output = NewType;

    fn add(self, rhs: Self) -> Self::Output {
        return NewType(self.0 + rhs.0);
    }
}


pub fn test() {
    banner::print_h0("New Type Idiom");
    println!("{}", std::any::type_name::<NewTypeSimple>());
    let new_type = NewTypeSimple(42);
    let new_type_as_primitive_1: i32 = new_type.0; // Tuple
    println!("new_type_as_primitive_1: {}", new_type_as_primitive_1);

    let NewTypeSimple(new_type_as_primitive_2) = new_type; // Destructuring
    println!("new_type_as_primitive_2: {}", new_type_as_primitive_2);

    // The new type idiom can greatly be enhanced by using traits
    banner::print_h0("New Type Idiom - Advanced");
    let new_type = NewType(42);
    println!("new_type: {:?}", new_type);
    println!("new_type immutable deref: {}", *new_type);

    let new_type_2 = NewType(13);
    let add_type = new_type + new_type_2;
    println!("add_type: {:?}", add_type);

}