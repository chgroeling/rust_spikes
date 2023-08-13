//! Experiments with Rc and Weak
//!

use std::rc::{Rc, Weak};

pub fn example_rc_weak() {
    println!("========================================");
    println!("{}", file!());
    println!("========================================");

    println!("----------------------------------------");
    println!("Experiment 1 - Rc and Weak");
    println!("----------------------------------------");

    let wobj : Weak<i32>; // create an empty weak reference
    {
        let content = 34;

        // Create reference counted object on the heap
        let obj1 = Rc::new(content);
        {
            wobj =Rc::downgrade(&obj1);
            println!(
                "strong_count: {}, weak_count: {}",
                Rc::strong_count(&obj1),
                Rc::weak_count(&obj1)
            );

            let obj2 = obj1.clone();

            println!(
                "strong_count: {}, weak_count: {}",
                Rc::strong_count(&obj1),
                Rc::weak_count(&obj1)
            );

            let _wobj1 = Rc::downgrade(&obj2); 

            println!(
                "strong_count: {}, weak_count: {}",
                Rc::strong_count(&obj1),
                Rc::weak_count(&obj1)
            );
            let upg = wobj.upgrade();
            println!(
                "weak deref: {:?}",upg
            );
        }
        println!(
            "strong_count: {}, weak_count: {}",
            Rc::strong_count(&obj1),
            Rc::weak_count(&obj1)
        );

        let upg = wobj.upgrade();
        println!(
            "weak deref: {:?}",upg
        );
    }
    let upg = wobj.upgrade();
    println!(
        "weak deref: {:?}",upg
    );
}
