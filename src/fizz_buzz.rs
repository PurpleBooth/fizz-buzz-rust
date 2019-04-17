pub fn fizz_buzz(value: i128) -> String {
    if value % 15 == 0 {
        return "fizz_buzz".to_string();
    }

    if value % 3 == 0 {
        return "fizz".to_string();
    }

    if value % 5 == 0 {
        return "buzz".to_string();
    }

    return value.to_string();
}

#[cfg(test)]
mod tests {
    use crate::fizz_buzz::fizz_buzz;

    #[test]
    fn it_returns_1_when_the_value_is_1() {
        assert_eq!(fizz_buzz(1), "1");
    }

    #[test]
    fn it_returns_2_when_the_value_is_2() {
        assert_eq!(fizz_buzz(2), "2");
    }

    #[test]
    fn it_returns_4_when_the_value_is_4() {
        assert_eq!(fizz_buzz(4), "4");
    }

    #[test]
    fn it_returns_fizz_when_the_value_is_3() {
        assert_eq!(fizz_buzz(3), "fizz");
    }

    #[test]
    fn it_returns_fizz_when_the_value_is_6() {
        assert_eq!(fizz_buzz(6), "fizz");
    }

    #[test]
    fn it_returns_fizz_when_the_value_is_9() {
        assert_eq!(fizz_buzz(9), "fizz");
    }

    #[test]
    fn it_returns_buzz_when_the_value_is_5() {
        assert_eq!(fizz_buzz(5), "buzz");
    }

    #[test]
    fn it_returns_buzz_when_the_value_is_10() {
        assert_eq!(fizz_buzz(10), "buzz");
    }

    #[test]
    fn it_returns_buzz_when_the_value_is_20() {
        assert_eq!(fizz_buzz(20), "buzz");
    }

    #[test]
    fn it_returns_fizz_buzz_when_the_value_is_15() {
        assert_eq!(fizz_buzz(15), "fizz_buzz");
    }

    #[test]
    fn it_returns_fizz_buzz_when_the_value_is_30() {
        assert_eq!(fizz_buzz(30), "fizz_buzz");
    }

    #[test]
    fn it_returns_fizz_buzz_when_the_value_is_45() {
        assert_eq!(fizz_buzz(45), "fizz_buzz");
    }
}
