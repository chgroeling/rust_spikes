//! Rust wrapper types experiments.
//!

mod banner;
mod rc_weak;
mod cell_refcell;

fn main() {
    rc_weak::example_rc_weak();
    cell_refcell::example_cell_refcell();
}
