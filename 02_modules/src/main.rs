// The mod blah; syntax imports a file for the compiler. You must use this on all 
// the files you want to compile.

mod mod_1;
mod mod_2;
mod mod_3;
mod mod_4;
mod mod_5;
mod test_mod;

fn main() {

    mod_1::mod_1(); 
    mod_2::mod_2();
    mod_3::mod_3();
    mod_4::mod_4();
    mod_5::mod_5();
}
