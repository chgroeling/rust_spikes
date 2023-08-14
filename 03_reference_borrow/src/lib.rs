//! This file gives some example what rust will do when working with references and mutable references
//! 
//! It contains not "real" tests. Just examples contained in a test.

/// An empty struct used as example material.
struct TestStruct {}

#[allow(dead_code)]
impl TestStruct {
    /// This function requests a mutable borrow to self.
    fn test_mutable_reference(&self) {
    }

    /// This function wants to move the ownership of self to this method.
    fn test_move_ownership(self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// This test is intended to show the usage of immutable borrows of a object.
    fn try_mulitple_immutable_borrows() {
        // multiple immutable borrows are allowed
        let a = 123;
        let a_1 = &a;
        let a_2 = &a;
        let a_3 = &a;
        // let a_4 = &mut a; // E0502 - not allowed

        assert_eq!(a, 123);
        assert_eq!(*a_1, 123); // no auto deref on i32 therefore asterik (*) is needed
        assert_eq!(*a_2, 123);
        assert_eq!(*a_3, 123);
    }

    #[test]
    /// This test is intended to show the usage of one mutable borrow of an object.
    fn try_one_mutable_borrow() {
        // one mutable borrows are allowed
        let mut a = 123;
        let a_1 = &mut a;
        // let a_2 = &mut a; // E0499 - not allowed
        // let a_3 = &a; // E0502 - not allowed

        *a_1 = 321;

        assert_eq!(a, 321);
    }

    #[test]
    /// This test shows what happens when the ownership moves. Especially in case of a method
    /// which is implemented for a struct.
    fn try_mutable_reference_and_move_ownership() {
        // difference between &self and self
        let lifetime1 = TestStruct {};

        lifetime1.test_mutable_reference();
        lifetime1.test_move_ownership();
        // This is not possible anymore:
        // lifetime1.test_move_ownership(); // E0382
        // The ownership was moved to test_move_ownership and here it has gone out of scope
    }
}
