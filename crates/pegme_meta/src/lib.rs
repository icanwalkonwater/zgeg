use pegme_core::{
    cst::ConcreteSyntaxTree,
    grammar::{Grammar, GrammarRule, PegExpression},
};

mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}
pub use parser::{parse as parse_as_cst, MetaPegmeKind};

#[cfg(test)]
mod tests;

pub fn parse(input: impl Into<String>) -> Result<Grammar, Box<dyn std::error::Error>> {
    let cst = parse_as_cst(input);
    Ok(cst_to_grammar(&cst))
}

pub fn cst_to_grammar(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> Grammar {
    assert_eq!(MetaPegmeKind::FILE, cst.kind().unwrap());

    let rules = cst
        .find_children_by_kind(MetaPegmeKind::ITEM_RULE)
        .map(|t| cst_to_rule(t))
        .collect();

    Grammar::from_rules(rules).unwrap()
}

fn cst_to_rule(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> GrammarRule {
    assert_eq!(MetaPegmeKind::ITEM_RULE, cst.kind().unwrap());

    let rule_kind = cst.find_child_by_kind(MetaPegmeKind::RULE_KIND).unwrap();
    let is_token = rule_kind.find_child_by_kind(MetaPegmeKind::TOKEN).is_some();

    let rule_name = cst.find_child_by_kind(MetaPegmeKind::IDENT).unwrap();
    let rule_body = cst.find_child_by_kind(MetaPegmeKind::EXPR).unwrap();

    let rule_name = cst_to_non_trivia_string(rule_name);
    let rule_body = cst_to_expr(rule_body);

    GrammarRule::new(rule_name, rule_body).with_token(is_token)
}

fn cst_to_non_trivia_string(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> String {
    let mut s = String::new();

    s.push_str(cst.text());
    for child in cst.children() {
        if child.is(MetaPegmeKind::TRIVIA) {
            continue;
        }

        s.push_str(&cst_to_non_trivia_string(child));
    }
    s
}

fn cst_to_string(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> String {
    let mut s = String::new();
    s.push_str(cst.text());
    for child in cst.children() {
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
    assert_eq!(MetaPegmeKind::EXPR, cst.kind().unwrap());
    cst_to_expr_choice(cst.find_child_by_kind(MetaPegmeKind::EXPR_CHOICE).unwrap())
}

fn cst_to_expr_choice(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::EXPR_CHOICE, cst.kind().unwrap());

    cst.find_children_by_kind(MetaPegmeKind::EXPR_SEQ)
        .map(|c| cst_to_expr_seq(c))
        .reduce(|l, r| l.or(r))
        .unwrap()
}

fn cst_to_expr_seq(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::EXPR_SEQ, cst.kind().unwrap());

    cst.find_children_by_kind(MetaPegmeKind::EXPR_PREDICATE)
        .map(|c| cst_to_expr_predicate(c))
        .reduce(|l, r| l.seq(r))
        .unwrap()
}

fn cst_to_expr_predicate(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::EXPR_PREDICATE, cst.kind().unwrap());

    let expr = cst_to_expr_repeat(cst.find_child_by_kind(MetaPegmeKind::EXPR_REPEAT).unwrap());

    let is_positive = cst.find_child_by_kind(MetaPegmeKind::AND).is_some();
    let is_negative = cst.find_child_by_kind(MetaPegmeKind::BANG).is_some();
    assert!(
        !(is_positive && is_negative),
        "Can't be postive and negative at the same time"
    );

    if is_positive || is_negative {
        expr.lookahead(is_positive)
    } else {
        expr
    }
}

fn cst_to_expr_repeat(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::EXPR_REPEAT, cst.kind().unwrap());

    let expr = cst_to_expr_atom(cst.find_child_by_kind(MetaPegmeKind::EXPR_ATOM).unwrap());

    if let Some(repeat_op_node) = cst.find_child_by_kind(MetaPegmeKind::REPEAT_OP) {
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

        if is_zero_more {
            expr.star()
        } else if is_one_more {
            expr.plus()
        } else if is_zero_one {
            expr.opt()
        } else {
            unreachable!()
        }
    } else {
        expr
    }
}

fn cst_to_expr_atom(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> PegExpression {
    assert_eq!(MetaPegmeKind::EXPR_ATOM, cst.kind().unwrap());

    if let Some(expr) = cst.find_child_by_kind(MetaPegmeKind::EXPR) {
        // Paren alternative.
        cst_to_expr(expr)
    } else if let Some(ident) = cst.find_child_by_kind(MetaPegmeKind::IDENT) {
        // Other rule.
        PegExpression::rule(&cst_to_non_trivia_string(ident))
    } else if let Some(_) = cst.find_child_by_kind(MetaPegmeKind::DOT) {
        // Any.
        PegExpression::any()
    } else if let Some(literal) = cst.find_child_by_kind(MetaPegmeKind::LITERAL) {
        // Literal.

        let literal = cst_to_non_trivia_string(literal);
        let literal = unescape_string(&literal[1..=literal.len() - 2]);

        PegExpression::literal(&literal)
    } else if let Some(class) = cst.find_child_by_kind(MetaPegmeKind::RANGES) {
        // Character ranges.

        let literals = class
            .find_children_by_kind(MetaPegmeKind::RANGE_SOLO)
            .map(|t| cst_to_character_range_ident(t))
            .map(|c| c..=c);

        let ranges = class
            .find_children_by_kind(MetaPegmeKind::RANGE_RANGE)
            .map(|range| {
                let mut iter = range.find_children_by_kind(MetaPegmeKind::RANGE_SOLO);
                let start = iter.next().unwrap();
                let end = iter.next().unwrap();
                assert_eq!(None, iter.next());

                cst_to_character_range_ident(start)..=cst_to_character_range_ident(end)
            });

        PegExpression::ranges(literals.chain(ranges))
    } else {
        unreachable!()
    }
}

fn cst_to_character_range_ident(cst: &ConcreteSyntaxTree<MetaPegmeKind>) -> char {
    let text = unescape_string(&cst_to_string(cst));
    assert!(text.len() == 1 || text.len() == 2);
    text.chars().last().unwrap()
}
