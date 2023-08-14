#[cfg(test)]
mod test_parse_and_extract_string {
    use crate::options::parse_and_extract;
    #[test]
    fn test0() -> () {
        assert_eq!(
            Ok(String::from("value")),
            parse_and_extract::<String>(&String::from("--key="), &String::from("--key=value"))
        );
    }
    #[test]
    fn test1() -> () {
        assert_eq!(
            Err("empty value"),
            parse_and_extract::<String>(&String::from("--key="), &String::from("--key="))
        );
    }
}
