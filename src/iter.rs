#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shows_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: "ABC".to_string(),
            },
            Shoe {
                size: 7,
                style: "XYZ".to_string(),
            },
            Shoe {
                size: 7,
                style: "ABC".to_string(),
            },
        ];
        let filtered = shows_in_my_size(shoes, 7);
        assert_eq!(
            filtered,
            vec![
                Shoe {
                    size: 7,
                    style: "XYZ".to_string(),
                },
                Shoe {
                    size: 7,
                    style: "ABC".to_string(),
                },
            ]
        )
    }

    #[test]
    fn calling_next() {
        let mut count = Counter::new();
        assert_eq!(count.next(), Some(1));
        assert_eq!(count.next(), Some(2));
        assert_eq!(count.next(), Some(3));
        assert_eq!(count.next(), Some(4));
        assert_eq!(count.next(), Some(5));
        assert_eq!(count.next(), None);
    }

    #[test]
    fn using_other_iterator_traits() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(x, y)| x * y)
            .filter(|predicate| predicate % 3 == 0)
            .sum();
        assert_eq!(sum, 18);
    }
}
