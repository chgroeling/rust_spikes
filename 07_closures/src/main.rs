mod banner;

fn main() {
    {
        banner::print_h0("Closures - non mutable");

        let a = 42;

        let closure = |x: i32| x * 2;

        let b = closure(a);

        println!("{a} -> {b}")
    }
    {
        banner::print_h0("Closures - non mutable ... with body");

        let a = 42;

        let closure = |x: i32| {
            let a = x + 2;
            a * 2
        };

        let b = closure(a);

        println!("{a} -> {b}")
    }

    {
        banner::print_h0("Closures - non mutable ... with body, explicit");

        let a = 42;

        let closure = |x: i32|->u32 {
            let a = x + 2;
            (a as u32)* 2
        };

        let b = closure(a);

        println!("{a} -> {b}")
    }

    {
        banner::print_h0("Closures - mutable ... with body");

        let a = 42;
        let mut mut_var: i32 = 30;

        let mut closure = |x: i32| {
            mut_var = mut_var + 1;
            let a = x + mut_var;
            a * 2
        };
        // mut_var = mut_var+1; <-- this is not allowed ... mut_var is borrowed

        let b = closure(a);
        mut_var = mut_var + 1; // <-- this is allowed becaus mut_var is not mutable borrowed anymore

        println!("{a}, {mut_var} -> {b}")
    }
}
