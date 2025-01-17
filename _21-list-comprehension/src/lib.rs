use comp_macro::comp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let c = comp!(x for x in [1, 2, 3]).collect::<Vec<_>>();
        assert_eq!(c, [1, 2, 3]);
    }

    #[test]
    fn test_2() {
        let c = comp!(x for x in [1, 2, 3, 0] if x < 2 if x > 0).collect::<Vec<_>>();
        assert_eq!(c, [1]);
    }

    #[test]
    fn test_3() {
        let c = comp![x * 4 for x in [1, 2, 3, 0]].collect::<Vec<_>>();
        assert_eq!(c, [4, 8, 12, 0]);
    }

    #[test]
    fn test_4() {
        #[derive(PartialEq, Debug)]
        struct Author {
            name: String,
            age: u8,
        }

        let c = comp![
            Author{name, age} for (name, age) in [("Franz Bonaparta".to_string(), 32u8)]
        ]
        .collect::<Vec<_>>();

        let authors = [
            Author {
                name: "Franz Bonaparta".to_string(),
                age: 32u8,
            },
        ];

        assert_eq!(c, authors);
    }
}
