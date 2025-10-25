use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub struct PackratParser<R = &'static str> {
    input: String,
    position: usize,
    memos: HashMap<(R, PackratMark), Option<PackratMark>>,
}

impl<R: Copy + Hash + Eq> PackratParser<R> {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            position: 0,
            memos: Default::default(),
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn mark(&self) -> PackratMark {
        PackratMark(self.position)
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }

    pub fn reset_to(&mut self, mark: PackratMark) {
        self.position = mark.0;
    }

    pub fn eat(&mut self, cond: impl FnOnce(char) -> bool) -> Option<char> {
        let c = self.input.chars().nth(self.position);
        match c {
            Some(c) if cond(c) => {
                self.position += 1;
                Some(c)
            }
            _ => None,
        }
    }

    pub fn anything(&mut self) -> Option<char> {
        self.eat(|_| true)
    }

    pub fn expect(&mut self, lit: &str) -> bool {
        if self.position < self.input.len() && self.input[self.position..].starts_with(lit) {
            self.position += lit.len();
            true
        } else {
            false
        }
    }

    pub fn eat_up_to(&mut self, mark: PackratMark) -> &str {
        assert!(self.position <= mark.0);
        let prev_position = self.position;
        self.reset_to(mark);
        &self.input[prev_position..mark.0]
    }

    pub fn memo(&mut self, rule: R, start: PackratMark) -> Option<Option<PackratMark>> {
        self.memos.get(&(rule, start)).cloned()
    }

    pub fn memoize_match(&mut self, rule: R, start: PackratMark, end: PackratMark) {
        assert!(start.0 <= end.0);
        self.memos.insert((rule, start), Some(end));
    }

    pub fn memoize_miss(&mut self, rule: R, start: PackratMark) {
        self.memos.insert((rule, start), None);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
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
        assert_eq!(parser.eat(|c| c.is_ascii()), Some('A'));
        assert_eq!(parser.position, 1);
        assert_eq!(parser.eat(|_| false), None);
        assert_eq!(parser.position, 1);
        assert_eq!(parser.eat(|c| c == 'B'), Some('B'));
        assert_eq!(parser.position, 2);
        assert_eq!(parser.eat(|c| c == 'C'), Some('C'));
        assert_eq!(parser.position, 3);
        assert_eq!(parser.eat(|c| c == 'D'), Some('D'));
        assert_eq!(parser.position, 4);
        assert_eq!(parser.eat(|_| true), None);
        assert_eq!(parser.position, 4);
        assert_eq!(parser.eat(|_| false), None);
        assert_eq!(parser.position, 4);
        assert_eq!(parser.eat(|c| c == 'D'), None);
        assert_eq!(parser.position, 4);
    }

    #[test]
    fn packrat_anything() {
        let mut parser = PackratParser::<()>::new("ABCD");
        assert_eq!(parser.position, 0);
        assert_eq!(parser.anything(), Some('A'));
        assert_eq!(parser.position, 1);
        assert_eq!(parser.anything(), Some('B'));
        assert_eq!(parser.position, 2);
        assert_eq!(parser.anything(), Some('C'));
        assert_eq!(parser.position, 3);
        assert_eq!(parser.anything(), Some('D'));
        assert_eq!(parser.position, 4);
        assert_eq!(parser.anything(), None);
        assert_eq!(parser.position, 4);
        assert_eq!(parser.anything(), None);
        assert_eq!(parser.position, 4);
    }

    #[test]
    fn packrat_mark() {
        let mut parser = PackratParser::<()>::new("ABCD");
        assert!(matches!(parser.mark(), PackratMark(0)));
        parser.anything();
        assert!(matches!(parser.mark(), PackratMark(1)));
        parser.anything();
        parser.anything();
        assert!(matches!(parser.mark(), PackratMark(3)));
    }

    #[test]
    fn packrat_reset_to_mark() {
        let mut parser = PackratParser::<()>::new("ABCD");
        assert_eq!(parser.anything(), Some('A'));
        let m = parser.mark();
        assert_eq!(parser.anything(), Some('B'));
        assert_eq!(parser.anything(), Some('C'));
        parser.reset_to(m);
        assert_eq!(parser.anything(), Some('B'));
        assert_eq!(parser.anything(), Some('C'));
    }

    #[test]
    fn packrat_expect() {
        let mut parser = PackratParser::<()>::new("ABCD");
        assert_eq!(parser.position, 0);
        assert!(parser.expect("AB"));
        assert_eq!(parser.position, 2);
        assert!(!parser.expect("D"));
        assert_eq!(parser.position, 2);
        assert!(parser.expect("C"));
        assert_eq!(parser.position, 3);
    }
}
