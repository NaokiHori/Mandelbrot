#[cfg(test)]
mod test_parse_and_extract_f64 {
    use crate::options::parse_and_extract;
    #[test]
    fn test0() -> () {
        assert_eq!(
            Ok(1.0e-7f64),
            parse_and_extract::<f64>(&String::from("--key="), &String::from("--key=1.e-7"))
        );
    }
    #[test]
    fn test1() -> () {
        assert_eq!(
            Err("empty value"),
            parse_and_extract::<f64>(&String::from("--key="), &String::from("--key="))
        );
    }
    #[test]
    fn test2() -> () {
        assert_eq!(
            Err("invalid value"),
            parse_and_extract::<f64>(&String::from("--key="), &String::from("--key=value"))
        );
    }
}
