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

struct MyError1 {
    msg: String,
}


impl MyErrorTrait for MyError1 {
    fn get_error_msg(&self) -> &str {
        return &self.msg;
    }
}


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

//--------------
pub fn fn_ret_and_traits() {
    banner::print_h0("Returning traits - statically and dynamically");

    println!("returning_result_static:{:?}", returning_result_static());
    println!("returning_result_dyn:{:?}", returning_result_dyn(true));
    println!("returning_result_dyn:{:?}", returning_result_dyn(false));
}