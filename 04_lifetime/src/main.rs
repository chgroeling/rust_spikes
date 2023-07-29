struct Lifetime1 {}

impl Lifetime1 {
    fn test_mutable_borrow(&mut self) {
        println!("This function wants a mutable borrow of self ")
    }
    fn test_move_ownership(mut self) {
        // the mut here is optional
        println!("This function moves self and makes the copy mutable");
    }
}

fn main() {
    {
        // difference between &mut and mut
        let mut lifetime1 = Lifetime1 {};

        lifetime1.test_mutable_borrow();
        lifetime1.test_move_ownership();
        // This is not possible anymore:
        // lifetime1.test_move_ownership();
        // The ownership was moved to test_move_ownership and here it has gone out of scope
    }
}
