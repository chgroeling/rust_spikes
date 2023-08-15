mod banner;
use banner::print_h0;

#[derive(Debug)]
#[allow(dead_code)]
enum SimpleEnum {
    Value1,
    Value2,
    Value3,
}

#[derive(Debug)]
#[allow(dead_code)]
enum IntEnum {
    Value1 = 11,
    Value2 = 12,
    Value3 = 13,
}

// Type inside enum
#[allow(dead_code)]
enum ComplexEnum {
    A(u32),
    B(i32),
    C(f32),
}

use ComplexEnum::*;

fn question_mark_op() -> Result<i32, i32> {
    let res: Result<i32, i32> = Err(42);

    let test_res = res?; //<-- this will return error
    println!("test_res:{}", test_res);
    Ok(10)
}

fn main() {
    {
        print_h0("Simple enum handling");
        let e_val = SimpleEnum::Value1; // assign enum
        println!("e_val = {:?}", e_val); // print debug
    }
    {
        print_h0("Enum match syntax");
        let e_val = SimpleEnum::Value2; // assign enum
        match e_val {
            SimpleEnum::Value1 => println!("Value1 selected"),
            _ => println!("Other selected"),
        }
    }
    {
        print_h0("Cast enum to underlying type");
        let e_val = IntEnum::Value2; // assign enum
        let e_val_cast = e_val as i32;
        println!("e_val_cast = {:?}", e_val_cast);
    }
    {
        print_h0("Complex enum destructuring with match (1)");
        let e_val = ComplexEnum::C(5.0); // assign enum
        let fl: f32 = match e_val {
            ComplexEnum::C(f_val) => f_val,
            _ => 0.0,
        };

        println!("fl = {:?}", fl);
    }
    {
        print_h0("Complex enum destructuring with match (2)");
        let e_val = ComplexEnum::C(5.0); // assign enum
        let fl: f32 = match e_val {
            C(f_val) => f_val, // possible due to use ComplexEnum::{*};
            _ => 0.0,
        };

        println!("fl = {:?}", fl);
    }
    {
        print_h0("Complex enum destructuring with if let syntax");
        let e_val = ComplexEnum::C(5.0); // assign enum
                                         // if let is needed because this is a refutable pattern.  If it were an irrefutable pattern let .. would be enough
        if let ComplexEnum::C(fl) = e_val {
            println!("fl = {:?}", fl);
        }
    }
    {
        print_h0("Std enums - Option");
        let foo = Some(43); // Option type can be deduced by Type given in Some

        println!("foo = {:?}", foo);

        if let Option::Some(result) = foo {
            println!("result = {:?}", result);
        }
    }
    {
        print_h0("Std enums - Result");
        let foo: Result<i32, i32> = Ok(23); // Result cannot be deduced because error Type is missing.
        println!("foo = {:?}", foo);

        if let Result::Ok(result) = foo {
            println!("result = {:?}", result);
        }
    }
    {
        print_h0("Std enums - Result - error propagation");
        let foo= question_mark_op(); 
        println!("foo = {:?}", foo);
    }
}
