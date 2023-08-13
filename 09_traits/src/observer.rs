//! Rust traits explained by a simple observer pattern implementation.
//!

/// This trait indicates that an object is observeable. The notify method is called by the Observer.
trait Observable {
    /// This method is called when the Observer notifies its clients.
    fn notify(&mut self);
}

/// Observerable Test Objekt 1
struct Object1 {}
impl Observable for Object1 {
    fn notify(&mut self) {
        println!("Object1")
    }
}

/// Observerable Test Objekt 2
struct Object2 {}
impl Observable for Object2 {
    fn notify(&mut self) {
        println!("Object2")
    }
}

/// Observerable Test Objekt 3
struct Object3 {
    test_int: i32,
}
impl Observable for Object3 {
    fn notify(&mut self) {
        self.test_int = self.test_int + 1;
        println!("Object3 mutated = {}", self.test_int)
    }
}

/// This is the actual observer which notifies all registered Observerables.
struct Observer<'a> {
    /// A Vector of SimpleObservables which will be all call when notifyall is called.
    ///
    /// The `dyn` keyword indicates that an Trait Object is requested.
    observers: Vec<&'a mut dyn Observable>,
}

impl<'a> Observer<'a> {
    fn subscribe(&mut self, obs: &'a mut dyn Observable) {
        self.observers.push(obs);
    }

    fn notifyall(&mut self) {
        for i in self.observers.iter_mut() {
            i.notify();
        }
    }
}

pub fn example_observer() {
    println!("========================================");
    println!("{}", file!());
    println!("========================================");
    
    let mut test_obj1 = Object1 {};
    let mut test_obj2 = Object2 {};
    let mut observer = Observer { observers: vec![] };

    observer.subscribe(&mut test_obj1);
    observer.subscribe(&mut test_obj2);

    {
        // open another scope

        // lifetime of test_obj3 is shorter than of test_obj1,2
        let mut test_obj3 = Object3 { test_int: 11 };

        observer.subscribe(&mut test_obj3);

        // test_obj3.test_int = 12; <-- this does not work ... its already borrowed. Therefore this pattern is not usable in production
        //
        // I assume that this is solvable by using Rc and Weak. Or maybe make all Observables non mutable
        // and use inner mutability. But nonetheless this example is a good explanation for how traits work.
        observer.notifyall();
    }

    //    observer.notifyall(); <-- does not work. Lifetime of test_obj3 is to short.
}
