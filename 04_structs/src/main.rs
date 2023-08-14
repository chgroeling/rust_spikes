//! This crate is used to do some experiments with structs and lifetime.  

mod banner;

/// A struct containing 3 value types
struct TestStruct1 {
    a: i32,
    b: i32, // a field cannot be mutable  (e.g. mut b:i32)  The mutability of a struct is in its binding:
    c: i32,
}

/// A struct containing 3 value types. It also implements a `new` method
struct TestStruct2 {
    a: i32,
    b: i32,
    c: i32,
}

impl TestStruct2 {
    /// Create a [TestStruct2] with values a=0, b=1, c=2
    fn new() -> TestStruct2 {
        TestStruct2 { a: 0, b: 1, c: 2 }
    }
}

/// A module that containts [structs::TestStruct3]
mod structs {
    /// Struct with private variables
    ///
    /// In Rust, a file is implicitly a module and all members in a struct are module wide public.
    /// Outside of the module everything is private. To make a member public use the keyword `pub`.
    /// This also must be done for the struct itself.
    pub struct TestStruct3 {
        a: i32,
        b: i32,
        pub c: i32,
    }

    impl TestStruct3 {
        pub fn new() -> TestStruct3 {
            TestStruct3 { a: 0, b: 1, c: 2 }
        }
        /// A borrow to self prevents that the ownership changes
        pub fn get(&self) -> (i32, i32, i32) {
            (self.a, self.b, self.c)
        }

        /// a mutable borrow is needed when one want to change a value
        pub fn set_b(&mut self, b: i32) {
            self.b = b
        }
    }
}

/// Struct with multiple mutable value types.
///
/// `'lifetime` is the so called lifetime annotation.  This means that the structs lifetime is connected
/// to the lifetime of `a`, `b` and `c`. E.g. if `a` goes out of scope the struct must go out of scope as well.
struct TestStruct4<'lifetime> {
    a: &'lifetime mut i32,
    b: &'lifetime mut i32,
    c: &'lifetime mut i32,
}

/// An example tuple struct containing 3 integers.
struct TestStruct5(i32, i32, i32);

/// A struct containing itself.
struct TestStruct6 {
    a: i32,
    b: Option<Box<TestStruct6>>,
}

fn main() {
    {
        banner::print_h0("Immutable struct. No field can be changed.");
        let test_struct1 = TestStruct1 {
            a: 10,
            b: 11,
            c: 12,
        };

        // this is not allowed. The struct is inmutable
        // test_struct1.b = 10
        println!(
            "test_struct1 -> a={0} b={1} c={2}",
            test_struct1.a, test_struct1.b, test_struct1.c
        );
    }
    {
        banner::print_h0("copy struct");
        let test_struct1 = TestStruct1 {
            a: 10,
            b: 11,
            c: 12,
        };

        // shadowing of test_struct1
        let test_struct1 = TestStruct1 {
            a: test_struct1.a,
            b: test_struct1.b + 10,
            c: test_struct1.c,
        };
        println!(
            "test_struct1 -> a={0} b={1} c={2}",
            test_struct1.a, test_struct1.b, test_struct1.c
        );
    }

    {
        banner::print_h0("update syntax ... does copy it too!");
        let test_struct1 = TestStruct1 {
            a: 10,
            b: 11,
            c: 12,
        };

        // shadowing of test_struct1
        let test_struct1 = TestStruct1 {
            b: test_struct1.b + 15,
            ..test_struct1
        };
        println!(
            "test_struct1 -> a={0} b={1} c={2}",
            test_struct1.a, test_struct1.b, test_struct1.c
        );
    }

    {
        banner::print_h0("Mutable struct ... mutability is inherited to all fields");
        let mut test_struct1 = TestStruct1 {
            a: 10,
            b: 11,
            c: 12,
        };

        test_struct1.b = 15;

        println!(
            "test_struct1 -> a={0} b={1} c={2}",
            test_struct1.a, test_struct1.b, test_struct1.c
        );
    }
    {
        banner::print_h0("Struct with method");
        let mut test_struct2 = TestStruct2::new(); // reference is moved

        test_struct2.b = 15; // ok because struct is mutable
        println!(
            "test_struct2 -> a={0} b={1} c={2}",
            test_struct2.a, test_struct2.b, test_struct2.c
        );
    }
    {
        banner::print_h0("Struct with private fields");
        let mut test_struct3 = structs::TestStruct3::new();

        // test_struct3.b = 15; field is private this is not allowed

        test_struct3.c = 33; // this field is public

        test_struct3.set_b(44);

        let (a, b, c) = test_struct3.get();

        println!("test_struct3 -> a={0} b={1} c={2}", a, b, c);
    }
    {
        banner::print_h0("Struct with mutable borrows");
        let mut b = 0;
        let mut c = 0;
        let mut a = 0;
        // The '_ means the compiler should infer the lifetime
        let mut test_struct4: TestStruct4<'_> = TestStruct4 {
            a: &mut a,
            b: &mut b,
            c: &mut c,
        };
        {
            let mut a_v2 = 0;
            test_struct4.a = &mut a_v2;
            *test_struct4.a = 12;
            *test_struct4.b = 14;
            *test_struct4.c = 16;

            println!("test_struct4 -> a={0} b={1} c={2} a_v2={3}", a, b, c, a_v2);
        }

        //  *test_struct4.a = 5; <-- this is not possible because ac does not life long enough
    }
    {
        banner::print_h0("Tuple  Struct");
        let test_struct5 = TestStruct5(32, 33, 34);

        println!(
            "test_struct5 -> 0={0} 1={1} 2={2}",
            test_struct5.0, test_struct5.1, test_struct5.2
        )
    }
    {
        banner::print_h0("Self containing struct");
        let test_struct6 = TestStruct6 {
            a: 20,
            b: Some(Box::new(TestStruct6 {
                a: 21,
                b: Some(Box::new(TestStruct6 { a: 21, b: None })),
            })),
        };

        let b = (&test_struct6.b).as_ref().unwrap();

        let a2 = b.a;

        let b2 = b.as_ref().b.as_ref().unwrap();

        let a3 = b2.a;

        println!(
            "test_struct6 -> 0={0} 1={1} 2={2}",
            test_struct6.a, a2, a3
        )
    }
}
