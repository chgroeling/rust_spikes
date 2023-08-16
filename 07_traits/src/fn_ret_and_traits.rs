use crate::banner;
use core::fmt::Debug;

#[derive(Debug)]
#[allow(dead_code)]
enum TestError {
    Error1,
    Error2,
    Error3(String)
}

fn returning_result_static() -> Result<i32, TestError> {
    Err(TestError::Error3("blabla".to_string()))
}

// -------------------------


trait MyErrorTrait {
    fn get_error_msg(&self) ->  &str ;
}


impl Debug for dyn MyErrorTrait {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MyErrorTrait: {}", self.get_error_msg())
    }
}

#[derive(Debug)]
struct MyError1 {
    msg: String,
}


impl MyErrorTrait for MyError1 {
    fn get_error_msg(&self) -> &str {
        return &self.msg;
    }
}

#[derive(Debug)]
struct MyError2 {
    msg: String,
}


impl MyErrorTrait for MyError2 {
    fn get_error_msg(&self) -> &str {
        return &self.msg;
    }
}

fn returning_result_dyn(switch: bool) -> Result<i32, Box<dyn MyErrorTrait>> {
    if switch {
        return Err(Box::new(MyError1{msg:"MyError1 - hello world".to_string()}));
    } else {
        return Err(Box::new(MyError2{msg:"MyError2 - hallo welt".to_string()}));
    }
}

// This "fn returning_result_static_2<T:MyErrorTrait>(switch: bool) -> Result<i32,T> " is not possible. 
// Reason: A user could call returning_result_static_2::<blabla>. But just the same class Err(MyError(...)) is returned. The 
// function should be univsal but its not.
//
// In argument position, impl Trait is very similar in semantics to a generic type parameter. However, there are 
// significant differences between the two in return position. With impl Trait, unlike with a generic type parameter, 
// the function chooses the return type, and the caller cannot choose the return type.
fn returning_result_static_2(switch: bool) -> Result<i32, impl MyErrorTrait+Debug>{ // MyErrorTrait and Debug should be realized 

    // interstingly the debug trait of MyError1 is called by returning_result_static 2

    if (switch) {
        return Err(MyError1{msg:"MyError1 - hello world".to_string()});
    } else {
        return Err(MyError1{msg:"MyError1 - hello world".to_string()}); // Return MyError2 here is not possible ... Statically only one type can be returned
    }

}

//--------------
pub fn fn_ret_and_traits() {
    banner::print_h0("Returning traits - statically and dynamically");

    println!("returning_result_static:{:?}", returning_result_static());
    println!("returning_result_dyn:{:?}", returning_result_dyn(true));
    println!("returning_result_dyn:{:?}", returning_result_dyn(false));
    println!("returning_result_static_2:{:?}", returning_result_static_2(false));
}