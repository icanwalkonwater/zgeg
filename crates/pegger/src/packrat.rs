use std::{collections::HashMap, fmt::Display, hash::Hash};

pub struct PackratParser<R = &'static str> {
    input: String,
    position: usize,
    memo: HashMap<(R, usize), bool>,
}

impl<R: Copy + Hash + Eq> PackratParser<R> {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            position: 0,
            memo: Default::default(),
        }
    }

    pub fn mark(&self) -> PackratMark {
        PackratMark(self.position)
    }

    pub fn reset_to(&mut self, mark: PackratMark) {
        self.position = mark.0;
    }

    pub fn eat(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.position);
        self.position += 1;
        c
    }

    pub fn memo(&self, rule: R, mark: PackratMark) -> Option<bool> {
        self.memo.get(&(rule, mark.0)).copied()
    }

    pub fn memoize(&mut self, rule: R, mark: PackratMark, value: bool) {
        self.memo.insert((rule, mark.0), value);
    }

    // Utils

    pub fn expect(&mut self, lit: &str) -> bool {
        if self.input[self.position..].starts_with(lit) {
            self.position += lit.len();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PackratMark(usize);

impl PackratMark {
    pub fn offset(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{PackratMark, PackratParser};

    #[test]
    fn packrat_eat() {
        let mut parser = PackratParser::<()>::new("ABCD");
        assert_eq!(parser.eat(), Some('A'));
        assert_eq!(parser.eat(), Some('B'));
        assert_eq!(parser.eat(), Some('C'));
        assert_eq!(parser.eat(), Some('D'));
        assert_eq!(parser.eat(), None);
        assert_eq!(parser.eat(), None);
    }

    #[test]
    fn packrat_mark() {
        let mut parser = PackratParser::<()>::new("ABCD");
        assert!(matches!(parser.mark(), PackratMark(0)));
        parser.eat();
        assert!(matches!(parser.mark(), PackratMark(1)));
        parser.eat();
        parser.eat();
        assert!(matches!(parser.mark(), PackratMark(3)));
    }

    #[test]
    fn packrat_rest_to_mark() {
        let mut parser = PackratParser::<()>::new("ABCD");
        assert_eq!(parser.eat(), Some('A'));
        let m = parser.mark();
        assert_eq!(parser.eat(), Some('B'));
        assert_eq!(parser.eat(), Some('C'));
        parser.reset_to(m);
        assert_eq!(parser.eat(), Some('B'));
        assert_eq!(parser.eat(), Some('C'));
    }

    #[test]
    fn packrat_expect() {
        let mut parser = PackratParser::<()>::new("ABCD");
        assert!(parser.expect("AB"));
        assert!(!parser.expect("D"));
        assert!(parser.expect("C"));
    }
}
