#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn try_convert_slice_to_string1() {
        let s1: String = "Hello World".to_string();

        assert_eq!(s1, "Hello World");
    }

    #[test]
    fn try_convert_slice_to_string2() {
        let s1: String = String::from("Hello World");

        assert_eq!(s1, "Hello World");
    }

    #[test]
    fn try_mutable_borrow_of_string() {
        let mut s1 = "Hello".to_string();
        let s_b = &mut s1;
        s_b.push_str(" World");

        assert_eq!(s1, "Hello World");
    }

    #[test]
    fn try_mutable_borrow_of_string2() {
        let mut s1 = "ABC".to_string();
        let mut s2 = "Hello".to_string();

        // this is interesting ... it deactivates the unused assignemnts warning of the following line
        #[allow(unused_assignments)]
        let mut s_b: &mut String = &mut s1;

        s_b = &mut s2;

        // this *s_b part can be inferred by the compiler
        (*s_b).push_str(" World");
        assert_eq!(s1, "ABC");
        assert_eq!(s2, "Hello World");
    }

    #[test]
    fn try_mutable_borrow_of_slice() {
        let mut s1 = "ABC";
        let mut s2 = "Hello";

        #[allow(unused_assignments)]
        let mut s_b = &mut s1;

        s_b = &mut s2;

        assert_eq!(*s_b, "Hello");
    }

    #[test]
    fn try_string_concat() {
        let mut s1 = "Hello ".to_string();
        let s2: &str = "World";

        s1.push_str(s2);
        assert_eq!(s1, "Hello World");
    }

    #[test]
    fn try_string_format() {
        let s1 = "Hello";
        let s2: &str = "World";

        let s3 = format!("{} {}", s1, s2);

        assert_eq!(s3, "Hello World");
    }

    #[test]
    fn try_string_slicing() {
        let s1 = "Hello World";
        let s2: &str = &s1[0..5];

        assert_eq!(s2, "Hello");
    }

    #[test]
    fn try_string_mut_slicing() {
        let mut s1 = "Hello World".to_string();
        let s2 = &mut s1[0..5];

        s2.make_ascii_uppercase();

        assert_eq!(s1, "HELLO World");
    }

}
