//! Experiments with Rc and Weak
//!

use crate::banner;
use std::cell::{Cell, RefCell, self};

struct CellTest {
    cell_i32: Cell<i32>,
}

struct RefCellTest {
    cell_i32: RefCell<i32>,
}

pub fn example_cell_refcell() {
    banner::print_h0(&format!("{}", file!()));
    banner::print_h1("Experiment 1 - Cell in immutable struct");

    let cell_test = CellTest { cell_i32: Cell::new(4) };
    cell_test.cell_i32.set(50);
    println!("cell_i32: {}", cell_test.cell_i32.get());
    
    banner::print_h1("Experiment 2 - RefCell in immutable struct");

    let cell_test = RefCellTest { cell_i32: RefCell::new(4) };
    { //<--- if this is let out the thread panics... Only one mut is allowed. This must first go out of scope.
        let mut mut_to_i32 = cell_test.cell_i32.borrow_mut();
        *mut_to_i32 = 42;
    }
    println!("cell_i32: {}", cell_test.cell_i32.borrow());
    
}
