use std::sync::Arc;

use crate::{
    cst::ConcreteSyntaxTree,
    meta::generated::{parse_rule, MetaPegmeKind, MetaPegmeKind::*},
};

fn cst_node<const N: usize>(
    kind: MetaPegmeKind,
    children: [ConcreteSyntaxTree<MetaPegmeKind>; N],
) -> ConcreteSyntaxTree<MetaPegmeKind> {
    ConcreteSyntaxTree::node(kind, children.into_iter().map(|c| Arc::new(c)).collect())
}

fn cst_leaf(s: &str) -> ConcreteSyntaxTree<MetaPegmeKind> {
    ConcreteSyntaxTree::leaf(s.into())
}

#[test]
fn expr_repeat_atom() {
    let cst = parse_rule("uwu", ExprRepeat);
    let expected = cst_node(
        ExprRepeat,
        [cst_node(
            ExprAtom,
            [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
        )],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_atom_paren() {
    let cst = parse_rule("(uwu)", ExprAtom);
    let expected = cst_node(
        ExprAtom,
        [
            cst_node(PAREN_L, [cst_node(Trivia, []), cst_leaf("(")]),
            cst_node(
                Expr,
                [cst_node(
                    ExprChoice,
                    [cst_node(
                        ExprSeq,
                        [cst_node(
                            ExprPredicate,
                            [cst_node(
                                ExprRepeat,
                                [cst_node(
                                    ExprAtom,
                                    [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
                                )],
                            )],
                        )],
                    )],
                )],
            ),
            cst_node(PAREN_R, [cst_node(Trivia, []), cst_leaf(")")]),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_atom_ident() {
    let cst = parse_rule("hello", ExprAtom);
    let expected = cst_node(
        ExprAtom,
        [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("hello")])],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_atom_dot() {
    let cst = parse_rule(".", ExprAtom);
    let expected = cst_node(
        ExprAtom,
        [cst_node(DOT, [cst_node(Trivia, []), cst_leaf(".")])],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_atom_keyword() {
    let cst = parse_rule("\"uwu\"", ExprAtom);
    let expected = cst_node(
        ExprAtom,
        [cst_node(
            Keyword,
            [cst_node(Trivia, []), cst_leaf("\"uwu\"")],
        )],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn trivia() {
    let cst = parse_rule("  \t\n \r\n // hello \n", Trivia);

    let expected = cst_node(
        Trivia,
        [
            cst_node(Whitespace, [cst_leaf("  \t\n \r\n ")]),
            cst_node(
                Comment,
                [cst_leaf("// hello "), cst_node(EOL, [cst_leaf("\n")])],
            ),
        ],
    );

    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn keyword_simple() {
    let cst = parse_rule("\"hello\"", Keyword);
    let expected = cst_node(Keyword, [cst_node(Trivia, []), cst_leaf("\"hello\"")]);
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn keyword_escaped() {
    let cst = parse_rule("\"he\\\"llo\"", Keyword);
    let expected = cst_node(Keyword, [cst_node(Trivia, []), cst_leaf("\"he\\\"llo\"")]);
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn character_ranges_simple() {
    let cst = parse_rule("[a]", CharacterRanges);
    let expected = cst_node(
        CharacterRanges,
        [
            cst_node(Trivia, []),
            cst_leaf("["),
            cst_node(CharacterRangesIdent, [cst_leaf("a")]),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn character_ranges_varied() {
    let cst = parse_rule("[a0_'\"]", CharacterRanges);
    let expected = cst_node(
        CharacterRanges,
        [
            cst_node(Trivia, []),
            cst_leaf("["),
            cst_node(CharacterRangesIdent, [cst_leaf("a")]),
            cst_node(CharacterRangesIdent, [cst_leaf("0")]),
            cst_node(CharacterRangesIdent, [cst_leaf("_")]),
            cst_node(CharacterRangesIdent, [cst_leaf("'")]),
            cst_node(CharacterRangesIdent, [cst_leaf("\"")]),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn character_ranges_escaped() {
    let cst = parse_rule("[\\]]", CharacterRanges);
    let expected = cst_node(
        CharacterRanges,
        [
            cst_node(Trivia, []),
            cst_leaf("["),
            cst_node(CharacterRangesIdent, [cst_leaf("\\]")]),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn character_ranges_escaped_range() {
    let cst = parse_rule("[\\[-\\]]", CharacterRanges);
    let expected = cst_node(
        CharacterRanges,
        [
            cst_node(Trivia, []),
            cst_leaf("["),
            cst_node(
                CharacterRangesRange,
                [
                    cst_node(CharacterRangesIdent, [cst_leaf("\\[")]),
                    cst_leaf("-"),
                    cst_node(CharacterRangesIdent, [cst_leaf("\\]")]),
                ],
            ),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn character_ranges_single_range() {
    let cst = parse_rule("[a-z]", CharacterRanges);
    let expected = cst_node(
        CharacterRanges,
        [
            cst_node(Trivia, []),
            cst_leaf("["),
            cst_node(
                CharacterRangesRange,
                [
                    cst_node(CharacterRangesIdent, [cst_leaf("a")]),
                    cst_leaf("-"),
                    cst_node(CharacterRangesIdent, [cst_leaf("z")]),
                ],
            ),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn character_ranges_multiple_range() {
    let cst = parse_rule("[a-z0-9]", CharacterRanges);
    let expected = cst_node(
        CharacterRanges,
        [
            cst_node(Trivia, []),
            cst_leaf("["),
            cst_node(
                CharacterRangesRange,
                [
                    cst_node(CharacterRangesIdent, [cst_leaf("a")]),
                    cst_leaf("-"),
                    cst_node(CharacterRangesIdent, [cst_leaf("z")]),
                ],
            ),
            cst_node(
                CharacterRangesRange,
                [
                    cst_node(CharacterRangesIdent, [cst_leaf("0")]),
                    cst_leaf("-"),
                    cst_node(CharacterRangesIdent, [cst_leaf("9")]),
                ],
            ),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn character_ranges_mixed() {
    let cst = parse_rule("[a-z012B-G]", CharacterRanges);
    let expected = cst_node(
        CharacterRanges,
        [
            cst_node(Trivia, []),
            cst_leaf("["),
            cst_node(
                CharacterRangesRange,
                [
                    cst_node(CharacterRangesIdent, [cst_leaf("a")]),
                    cst_leaf("-"),
                    cst_node(CharacterRangesIdent, [cst_leaf("z")]),
                ],
            ),
            cst_node(CharacterRangesIdent, [cst_leaf("0")]),
            cst_node(CharacterRangesIdent, [cst_leaf("1")]),
            cst_node(CharacterRangesIdent, [cst_leaf("2")]),
            cst_node(
                CharacterRangesRange,
                [
                    cst_node(CharacterRangesIdent, [cst_leaf("B")]),
                    cst_leaf("-"),
                    cst_node(CharacterRangesIdent, [cst_leaf("G")]),
                ],
            ),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn character_ranges_dashdashdashdash() {
    let cst = parse_rule("[----]", CharacterRanges);
    let expected = cst_node(
        CharacterRanges,
        [
            cst_node(Trivia, []),
            cst_leaf("["),
            cst_node(
                CharacterRangesRange,
                [
                    cst_node(CharacterRangesIdent, [cst_leaf("-")]),
                    cst_leaf("-"),
                    cst_node(CharacterRangesIdent, [cst_leaf("-")]),
                ],
            ),
            cst_node(CharacterRangesIdent, [cst_leaf("-")]),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn ident_alphabetic() {
    let cst = parse_rule("hello", IDENT);
    let expected = cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("hello")]);
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn ident_alphanumeric() {
    let cst = parse_rule("a123", IDENT);
    let expected = cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("a123")]);
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn ident_bs() {
    let cst = parse_rule("_a1_23", IDENT);
    let expected = cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("_a1_23")]);
    pretty_assertions::assert_eq!(*cst, expected);
}
