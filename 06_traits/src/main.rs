//! Rust traits explained by a simple observer pattern implementation.
//!
//! 

mod fn_ret_and_traits;
mod extension_traits;
mod banner;
mod observer;
mod static_lifetime;

fn main() {
    observer::example_observer();
    fn_ret_and_traits::fn_ret_and_traits();
    extension_traits::extension_traits();
    static_lifetime::static_lifetime();
}
