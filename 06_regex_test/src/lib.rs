#[cfg(test)]
mod tests {
    use regex::Regex;
 
    #[test]
    fn try_is_match_functionality() {
        // Check if regex matches
        let re = Regex::new(r"Hello \w+!").unwrap();
        let result = re.is_match("Hello World!");

        assert!(result);
    }

    #[test]
    fn try_an_regex_which_matches_the_start_of_a_string() {
        // This regex matches the following haystack
        let re = Regex::new(r"^[k]\w+").unwrap();
        let result = re.find("keyword bla bla");
        assert_eq!(result.unwrap().as_str(), "keyword");
    }

    #[test]
    fn try_what_happens_when_a_regex_doesnt_match() {
        // This regex does not match
        let re = Regex::new(r"^[b]\w+").unwrap();
        let result = re.find("keyword bla bla");
        assert!(result.is_none());
    }

    #[test]
    fn try_split_by_regex() {
        // split the string by one or more commas
        let re = Regex::new(r"[,]+").unwrap();
        let result : Vec<&str>= re.split("keyword,bla,,bla").collect();
        assert_eq!(result, vec!["keyword","bla","bla"]);
    }

    #[test]
    fn try_extract_named_capture_groups() {
        // Extract a capture group
        let re = Regex::new(r"Hello (?<name>\w+)!").unwrap();
        let caps = re.captures("Hello Murphy!").unwrap();
        println!("The name is: {}", &caps["name"]);

        assert_eq!(&caps["name"], "Murphy")
    }

    #[test]
    fn try_iterate_over_all_matches() {
        // Find everything that does not contain "l" and "o"
        let re: Regex = Regex::new(r"[^lo]").unwrap();
        let haystack = "Hello World";
        let matches: Vec<&str> = re.find_iter(haystack).map(|m| m.as_str()).collect();

        assert_eq!(matches, vec!["H", "e", " ", "W", "r", "d"]);
    }

    #[test]
    fn try_replace_a_unamed_capture_group() {
        let re = Regex::new(r"(Hello) \w+!").unwrap();
        let haystack = "Hello Murphy!";

        // here a clone on write is returned. In this case the string is a new onee
        // and clone was performed
        let result = re.replace(haystack, "$1 Christian!");

        assert!(matches!(result, std::borrow::Cow::Owned(_)));
        assert_eq!(result, "Hello Christian!");
    }

    #[test]
    fn try_replace_a_unamed_capture_group_but_fail_to_do_so() {
        let re = Regex::new(r"(wrong) \w+!").unwrap();
        let haystack = "Hello Murphy!";

        // try to replace but fail
        let result = re.replace(haystack, "$1 Christian!");

        // now result returns a borrowed pointer because it changed nothing
        assert!(matches!(result, std::borrow::Cow::Borrowed(_)));
        assert_eq!(result, "Hello Murphy!");
    }

}
