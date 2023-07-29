struct MoveCopyTest1 {}

impl MoveCopyTest1 {
    fn test_mutable_borrow(&self) {
        println!("This function wants a mutable borrow of self ")
    }
    fn test_move_ownership(self) {
        println!("This function moves self and makes the copy mutable");
    }
}

fn main() {
    {
        // difference between &self and self
        let lifetime1 = MoveCopyTest1 {};

        lifetime1.test_mutable_borrow();
        lifetime1.test_move_ownership();
        // This is not possible anymore:
        // lifetime1.test_move_ownership();
        // The ownership was moved to test_move_ownership and here it has gone out of scope
    }
}
