use super::{
    visit::{self, PegExpressionVisitorMut},
    PegExpression,
};

pub struct PegExpressionSimplifier;

impl PegExpressionVisitorMut for PegExpressionSimplifier {
    fn visit_expr_mut(&mut self, expr: &mut PegExpression) {
        match expr {
            PegExpression::Seq(left, right) => {
                // Simplify children first
                self.visit_expr_mut(left);
                self.visit_expr_mut(right);
                match (&**left, &**right) {
                    (PegExpression::Epsilon, e) | (e, PegExpression::Epsilon) => *expr = e.clone(),
                    _ => {}
                }
            }
            // Call default
            _ => visit::visit_expr_mut(self, expr),
        }
    }
}
