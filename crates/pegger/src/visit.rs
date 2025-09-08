use crate::{PegExpression, PegLiteralCharacterClass, PegRule, PegRuleName};

#[allow(unused_variables)]
pub trait PegExpressionVisitor {
    fn visit_literal_keyword(&mut self, keyword: &mut &'static str) {}
    fn visit_literal_range(&mut self, from: &mut char, to: &mut char) {}
    fn visit_literal_class(&mut self, class: &mut PegLiteralCharacterClass) {}
    fn visit_rule(&mut self, name: &mut PegRuleName) {}
    fn visit_seq(&mut self, left: &mut PegExpression, right: &mut PegExpression) {}
    fn visit_choice(&mut self, left: &mut PegExpression, right: &mut PegExpression) {}
    fn visit_repetition(&mut self, expr: &mut PegExpression, min: &mut u32, max: &mut Option<u32>) {
    }
    fn visit_predicate(&mut self, pred: &mut PegExpression, positive: &mut bool) {}
    fn visit_anything(&mut self) {}
    fn visit_nothing(&mut self) {}
}

pub fn visit_peg_expression(expr: &mut PegExpression, visitor: &mut impl PegExpressionVisitor) {
    use PegExpression::*;
    match expr {
        LiteralExact(keyword) => visitor.visit_literal_keyword(keyword),
        LiteralRange { from, to } => visitor.visit_literal_range(from, to),
        LiteralClass(class) => visitor.visit_literal_class(class),
        Rule(name) => visitor.visit_rule(name),
        Seq(left, right) => {
            visitor.visit_seq(left, right);
            visit_peg_expression(left, visitor);
            visit_peg_expression(right, visitor);
        }
        Choice(left, right) => {
            visitor.visit_choice(left, right);
            visit_peg_expression(left, visitor);
            visit_peg_expression(right, visitor);
        }
        Repetition { expr, min, max } => {
            visitor.visit_repetition(expr, min, max);
            visit_peg_expression(expr, visitor);
        }
        Predicate {
            expr: pred,
            positive,
        } => {
            visitor.visit_predicate(pred, positive);
            visit_peg_expression(pred, visitor);
        }
        Anything => visitor.visit_anything(),
        Nothing => visitor.visit_nothing(),
    }
}

pub fn visit_peg_rule(rule: &mut PegRule, visitor: &mut impl PegExpressionVisitor) {
    for expr in &mut rule.choices {
        visit_peg_expression(expr, visitor);
    }
}
