use std::{collections::HashMap, hash::Hash};

pub struct PackratParser<R = &'static str, D = bool> {
    input: String,
    position: usize,
    memos: HashMap<(R, PackratMark), Option<(PackratMark, D)>>,
}

impl<R: Copy + Hash + Eq, D: Clone> PackratParser<R, D> {
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

    pub fn advance(&mut self, by: usize) {
        self.position += by;
    }

    pub fn memo(&mut self, rule: R, start: PackratMark) -> Option<Option<(PackratMark, D)>> {
        self.memos.get(&(rule, start)).cloned()
    }

    pub fn memoize_match(&mut self, rule: R, start: PackratMark, end: PackratMark, value: D) {
        assert!(start.0 <= end.0);
        self.memos.insert((rule, start), Some((end, value)));
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
        assert_eq!(parser.anything(), Some('A'));
        assert_eq!(parser.anything(), Some('B'));
        assert_eq!(parser.anything(), Some('C'));
        assert_eq!(parser.anything(), Some('D'));
        assert_eq!(parser.anything(), None);
        assert_eq!(parser.anything(), None);
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
    fn packrat_rest_to_mark() {
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
        assert!(parser.expect("AB"));
        assert!(!parser.expect("D"));
        assert!(parser.expect("C"));
    }
}
