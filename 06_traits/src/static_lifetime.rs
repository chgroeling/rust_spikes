use std::fmt::Debug;

use crate::banner;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

pub fn static_lifetime() {
    banner::print_h0("Lifetime attribute 'static in traits");

    
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    // print_it(&i);
}