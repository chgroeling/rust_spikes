use crate::banner;

trait MyTrait {
    fn hello_world(&self);
}

struct MyTraitStruct {
}

impl MyTrait for MyTraitStruct {
    fn hello_world(&self) {
        println!("Original");
    }
}

trait MyExtension : MyTrait {
    fn hallo_welt(&self);

    fn hallo_welt2(&self) {
        println!("Extension2");
        self.hello_world(); // <-- for this the inheritance of MyTrait is needed
    }
}

impl <T: MyTrait> MyExtension for T { // Blanket implementation for MyTrait
    fn hallo_welt(&self) {
        println!("Extension1");
        self.hello_world();
    }
}

pub fn extension_traits() {
    banner::print_h0("Extension traits");
    let my_trait_struct = MyTraitStruct{};
    my_trait_struct.hallo_welt();
    my_trait_struct.hallo_welt2();
}