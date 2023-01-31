#[cfg(test)]
mod tests {
    use crate::helper_methods::*;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_convert_to_minutes() {
        let time = " 3 Std.";
        assert_eq!(convert_to_minutes(time), 180);
    }

    #[test]
    fn test_convert_to_minutes_2() {
        let time = " 3 Std. 10 Min";
        assert_eq!(convert_to_minutes(time), 190);
    }

    #[test]
    fn test_convert_to_minutes_3() {
        let time = " 3 Min";
        assert_eq!(convert_to_minutes(time), 3);
    }

    #[test]
    fn test_convert_to_minutes_4() {
        let time = "2 Std. 30 Min";
        assert_eq!(convert_to_minutes(time), 150);
    }

    #[test]
    fn test_string_to_difficulty() {
        let difficulty = "einfach";
        assert_eq!(string_to_difficulty(difficulty), Difficulty::Easy);
    }

    #[test]
    fn test_string_to_difficulty_2() {
        let difficulty = "medium";
        assert_eq!(string_to_difficulty(difficulty), Difficulty::Medium);
    }

    #[test]
    fn test_string_to_difficulty_3() {
        let difficulty = "schwer";
        assert_eq!(string_to_difficulty(difficulty), Difficulty::Hard);
    }
}
