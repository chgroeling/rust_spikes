mod banner;

static NUM:i32 = 30; // global

#[allow(dead_code)]
const CONST:i32= 31; // constant


fn work_with_static<'a>(arg:&'a i32) -> &'a i32 {
    // Returns a reference to `a_static` where its `'static`
    // lifetime is coerced to that of the input argument.
    return &NUM;
}
fn main() {

    {
        banner::print_h0("Static lifetime");


        let i_static_1: &'static i32 = &NUM;
        let str_static: &'static str = "Hello world";
        let i_static_2: &'static i32 = &5; // reference to a constant number in program memory

        {
            let _i : i32 = 50;
            let _b = work_with_static(&NUM);
        }
        // a_static stays accessible
        println!("{i_static_1} {str_static} {i_static_2}");
    }
}
