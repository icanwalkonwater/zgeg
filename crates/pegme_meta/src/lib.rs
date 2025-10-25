use indexmap::IndexMap;
use pegme_core::{
    cst::ConcreteSyntaxTree,
    grammar::{PegExpression, PegGrammar, PegRule, PegRuleName},
};

mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}
pub use parser::{parse, MetaPegmeKind};

#[cfg(test)]
mod tests;

pub fn cst_to_grammar(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegGrammar {
    assert_eq!(MetaPegmeKind::File, cst.kind().unwrap());

    let rules = cst
        .find_children_by_kind(MetaPegmeKind::Rule)
        .map(|t| cst_to_rule(t))
        .collect::<IndexMap<_, _>>();

    PegGrammar::new(rules).unwrap()
}

fn cst_to_rule(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> (PegRuleName, PegRule) {
    assert_eq!(MetaPegmeKind::Rule, cst.kind().unwrap());

    let rule_name = cst.find_child_by_kind(MetaPegmeKind::IDENT).unwrap();
    let rule_body = cst.find_child_by_kind(MetaPegmeKind::Expr).unwrap();

    let rule_name = PegRuleName(cst_to_non_trivia_string(rule_name).into());
    let rule_body = cst_to_expr(rule_body);

    (rule_name, PegRule::simple(rule_body))
}

fn cst_to_non_trivia_string(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> String {
    let mut s = String::new();

    s.push_str(cst.text());
    for child in cst.iter_children() {
        if child.is(MetaPegmeKind::Trivia) {
            continue;
        }

        s.push_str(&cst_to_non_trivia_string(child));
    }
    s
}

fn cst_to_string(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> String {
    let mut s = String::new();
    s.push_str(cst.text());
    for child in cst.iter_children() {
        s.push_str(&cst_to_non_trivia_string(child));
    }
    s
}

fn unescape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());

    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '\\' {
            out.push(match it.next().expect(&format!("What: {s}")) {
                't' => '\t',
                'n' => '\n',
                'r' => '\r',
                c => c,
            })
        } else {
            out.push(c);
        }
    }

    out
}

fn cst_to_expr(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::Expr, cst.kind().unwrap());
    cst_to_expr_choice(cst.find_child_by_kind(MetaPegmeKind::ExprChoice).unwrap())
}

fn cst_to_expr_choice(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::ExprChoice, cst.kind().unwrap());

    cst.find_children_by_kind(MetaPegmeKind::ExprSeq)
        .map(|c| cst_to_expr_seq(c))
        .reduce(|l, r| PegExpression::choice(l, r))
        .unwrap()
}

fn cst_to_expr_seq(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::ExprSeq, cst.kind().unwrap());

    cst.find_children_by_kind(MetaPegmeKind::ExprPredicate)
        .map(|c| cst_to_expr_predicate(c))
        .reduce(|l, r| PegExpression::seq(l, r))
        .unwrap()
}

fn cst_to_expr_predicate(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::ExprPredicate, cst.kind().unwrap());

    let expr = cst_to_expr_repeat(cst.find_child_by_kind(MetaPegmeKind::ExprRepeat).unwrap());

    let is_positive = cst.find_child_by_kind(MetaPegmeKind::AMPERSAND).is_some();
    let is_negative = cst.find_child_by_kind(MetaPegmeKind::EXCLAMATION).is_some();
    assert!(
        !(is_positive && is_negative),
        "Can't be postive and negative at the same time"
    );

    if is_positive || is_negative {
        PegExpression::predicate(expr, is_positive)
    } else {
        expr
    }
}

fn cst_to_expr_repeat(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::ExprRepeat, cst.kind().unwrap());

    let expr = cst_to_expr_atom(cst.find_child_by_kind(MetaPegmeKind::ExprAtom).unwrap());

    if let Some(repeat_op_node) = cst.find_child_by_kind(MetaPegmeKind::RepeatOp) {
        let is_zero_more = repeat_op_node
            .find_child_by_kind(MetaPegmeKind::STAR)
            .is_some();
        let is_one_more = repeat_op_node
            .find_child_by_kind(MetaPegmeKind::PLUS)
            .is_some();
        let is_zero_one = repeat_op_node
            .find_child_by_kind(MetaPegmeKind::QUESTION)
            .is_some();

        assert!(is_zero_more ^ is_one_more ^ is_zero_one);

        let min = if is_one_more { 1 } else { 0 };
        let max = if is_zero_one { Some(1) } else { None };

        PegExpression::repetition(expr, min, max)
    } else {
        expr
    }
}

fn cst_to_expr_atom(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::ExprAtom, cst.kind().unwrap());

    if let Some(expr) = cst.find_child_by_kind(MetaPegmeKind::Expr) {
        // Paren alternative.
        cst_to_expr(expr)
    } else if let Some(ident) = cst.find_child_by_kind(MetaPegmeKind::IDENT) {
        // Other rule.
        PegExpression::rule(&cst_to_non_trivia_string(ident))
    } else if let Some(_) = cst.find_child_by_kind(MetaPegmeKind::DOT) {
        // Any.
        PegExpression::Anything
    } else if let Some(keyword) = cst.find_child_by_kind(MetaPegmeKind::LITERAL_KEYWORD) {
        // Keyword.

        let keyword = cst_to_non_trivia_string(keyword);
        let keyword = unescape_string(&keyword[1..=keyword.len() - 2]);

        PegExpression::exact(keyword)
    } else if let Some(class) = cst.find_child_by_kind(MetaPegmeKind::CharacterRanges) {
        // Character class.

        let literals = class
            .find_children_by_kind(MetaPegmeKind::CharacterRangesIdent)
            .map(|t| cst_to_character_range_ident(t))
            .map(|c| (c, c));

        let ranges = class
            .find_children_by_kind(MetaPegmeKind::CharacterRangesRange)
            .map(|range| {
                let mut iter = range.find_children_by_kind(MetaPegmeKind::CharacterRangesIdent);
                let start = iter.next().unwrap();
                let end = iter.next().unwrap();
                assert_eq!(None, iter.next());

                (
                    cst_to_character_range_ident(start),
                    cst_to_character_range_ident(end),
                )
            });

        PegExpression::ranges(literals.chain(ranges).collect())
    } else {
        unreachable!()
    }
}

fn cst_to_character_range_ident(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> char {
    let text = unescape_string(&cst_to_string(cst));
    assert!(text.len() == 1 || text.len() == 2);
    text.chars().last().unwrap()
}
