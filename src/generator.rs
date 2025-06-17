pub struct Generator {
    pub current: usize,
    pub max: usize,
    pub match_against: Vec<(String, usize)>,
}

impl Generator {
    pub fn new(min: Option<usize>, max: usize, matches: Vec<(String, usize)>) -> Self {
        Self {
            current: min.unwrap_or(0),
            max,
            match_against: matches,
        }
    }
}

impl ExactSizeIterator for Generator {
    fn len(&self) -> usize {
        self.max - self.current
    }
}

impl Iterator for Generator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max != 0 && self.current == self.max {
            return None;
        };

        self.current += 1;

        let mut output = String::new();

        for (value, num_to_match) in &self.match_against {
            if self.current % num_to_match == 0 {
                output += value;
            }
        }

        if output.is_empty() {
            output = self.current.to_string();
        }

        Some(output)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.max - self.current, Some(self.max - self.current))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_until_10() {
        let fizzbuzz_matcher = vec![("Fizz".to_string(), 3), ("Buzz".to_string(), 5)];

        let mut gen = Generator {
            current: 0,
            max: 10,
            match_against: fizzbuzz_matcher,
        };

        let mut output = String::new();

        for _ in 0..10 {
            output += &gen.next().unwrap();
            output += "\n"
        }

        pretty_assertions::assert_eq!(output, "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n");
    }

    #[test]
    fn can_generate_until_15() {
        let fizzbuzz_matcher = vec![("Fizz".to_string(), 3), ("Buzz".to_string(), 5)];

        let mut gen = Generator {
            current: 0,
            max: 15,
            match_against: fizzbuzz_matcher,
        };

        let mut output = String::new();

        for _ in 0..15 {
            output += &gen.next().unwrap();
            output += "\n"
        }

        pretty_assertions::assert_eq!(
            output,
            "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n"
        );
    }

    #[test]
    fn can_do_fuzz() {
        let fizzbuzzfuzz_matcher = vec![
            ("Fizz".to_string(), 3),
            ("Buzz".to_string(), 5),
            ("Fuzz".to_string(), 7),
        ];

        let mut gen = Generator {
            current: 0,
            max: 15,
            match_against: fizzbuzzfuzz_matcher,
        };

        let mut output = String::new();

        for _ in 0..15 {
            output += &gen.next().unwrap();
            output += "\n"
        }

        pretty_assertions::assert_eq!(
            output,
            "1\n2\nFizz\n4\nBuzz\nFizz\nFuzz\n8\nFizz\nBuzz\n11\nFizz\n13\nFuzz\nFizzBuzz\n"
        );
    }
}
