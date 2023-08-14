#[cfg(test)]
mod test_parse_and_extract_usize {
    use crate::options::parse_and_extract;
    #[test]
    fn test0() -> () {
        assert_eq!(
            Ok(16384usize),
            parse_and_extract::<usize>(&String::from("--key="), &String::from("--key=16384"))
        );
    }
    #[test]
    fn test1() -> () {
        assert_eq!(
            Err("empty value"),
            parse_and_extract::<usize>(&String::from("--key="), &String::from("--key="))
        );
    }
    #[test]
    fn test2() -> () {
        assert_eq!(
            Err("invalid value"),
            parse_and_extract::<usize>(&String::from("--key="), &String::from("--key=-1"))
        );
    }
    #[test]
    fn test3() -> () {
        assert_eq!(
            Err("invalid value"),
            parse_and_extract::<usize>(&String::from("--key="), &String::from("--key=value"))
        );
    }
}
