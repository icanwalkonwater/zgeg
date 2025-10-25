use std::sync::Arc;

use pegme_core::cst::ConcreteSyntaxTree;

use crate::{
    parse_rule,
    MetaPegmeKind::{self, *},
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
fn file_empty() {
    let cst = parse_rule("", File);
    let expected = cst_node(File, [cst_node(Trivia, []), cst_node(EOF, [])]);
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn file_two_rules() {
    let cst = parse_rule("// Hello\nrule One = a;\nrule Two = b;", File);
    let expected = cst_node(
        File,
        [
            cst_node(
                Rule,
                [
                    cst_node(
                        RuleKind,
                        [cst_node(
                            RULE,
                            [
                                cst_node(
                                    Trivia,
                                    [cst_node(
                                        Comment,
                                        [cst_leaf("// Hello"), cst_node(EOL, [cst_leaf("\n")])],
                                    )],
                                ),
                                cst_leaf("rule"),
                            ],
                        )],
                    ),
                    cst_node(
                        IDENT,
                        [
                            cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                            cst_leaf("One"),
                        ],
                    ),
                    cst_node(
                        EQUAL,
                        [
                            cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                            cst_leaf("="),
                        ],
                    ),
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
                                            [cst_node(
                                                IDENT,
                                                [
                                                    cst_node(
                                                        Trivia,
                                                        [cst_node(Whitespace, [cst_leaf(" ")])],
                                                    ),
                                                    cst_leaf("a"),
                                                ],
                                            )],
                                        )],
                                    )],
                                )],
                            )],
                        )],
                    ),
                    cst_node(SEMICOLON, [cst_node(Trivia, []), cst_leaf(";")]),
                ],
            ),
            cst_node(
                Rule,
                [
                    cst_node(
                        RuleKind,
                        [cst_node(
                            RULE,
                            [
                                cst_node(Trivia, [cst_node(Whitespace, [cst_leaf("\n")])]),
                                cst_leaf("rule"),
                            ],
                        )],
                    ),
                    cst_node(
                        IDENT,
                        [
                            cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                            cst_leaf("Two"),
                        ],
                    ),
                    cst_node(
                        EQUAL,
                        [
                            cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                            cst_leaf("="),
                        ],
                    ),
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
                                            [cst_node(
                                                IDENT,
                                                [
                                                    cst_node(
                                                        Trivia,
                                                        [cst_node(Whitespace, [cst_leaf(" ")])],
                                                    ),
                                                    cst_leaf("b"),
                                                ],
                                            )],
                                        )],
                                    )],
                                )],
                            )],
                        )],
                    ),
                    cst_node(SEMICOLON, [cst_node(Trivia, []), cst_leaf(";")]),
                ],
            ),
            cst_node(Trivia, []),
            cst_node(EOF, []),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn rule_simple() {
    let cst = parse_rule("rule Hello = Hello;", Rule);
    let expected = cst_node(
        Rule,
        [
            cst_node(
                RuleKind,
                [cst_node(RULE, [cst_node(Trivia, []), cst_leaf("rule")])],
            ),
            cst_node(
                IDENT,
                [
                    cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                    cst_leaf("Hello"),
                ],
            ),
            cst_node(
                EQUAL,
                [
                    cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                    cst_leaf("="),
                ],
            ),
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
                                    [cst_node(
                                        IDENT,
                                        [
                                            cst_node(
                                                Trivia,
                                                [cst_node(Whitespace, [cst_leaf(" ")])],
                                            ),
                                            cst_leaf("Hello"),
                                        ],
                                    )],
                                )],
                            )],
                        )],
                    )],
                )],
            ),
            cst_node(SEMICOLON, [cst_node(Trivia, []), cst_leaf(";")]),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_atom() {
    let cst = parse_rule("uwu", Expr);
    let expected = cst_node(
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
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_bullshit() {
    let cst = parse_rule("a b / !c d d* (e / &f g)", Expr);
    let expected = cst_node(
        Expr,
        [cst_node(
            ExprChoice,
            [
                // a b
                cst_node(
                    ExprSeq,
                    [
                        // a
                        cst_node(
                            ExprPredicate,
                            [cst_node(
                                ExprRepeat,
                                [cst_node(
                                    ExprAtom,
                                    [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("a")])],
                                )],
                            )],
                        ),
                        // b
                        cst_node(
                            ExprPredicate,
                            [cst_node(
                                ExprRepeat,
                                [cst_node(
                                    ExprAtom,
                                    [cst_node(
                                        IDENT,
                                        [
                                            cst_node(
                                                Trivia,
                                                [cst_node(Whitespace, [cst_leaf(" ")])],
                                            ),
                                            cst_leaf("b"),
                                        ],
                                    )],
                                )],
                            )],
                        ),
                    ],
                ),
                cst_node(
                    SLASH_F,
                    [
                        cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                        cst_leaf("/"),
                    ],
                ),
                cst_node(
                    ExprSeq,
                    [
                        // !c
                        cst_node(
                            ExprPredicate,
                            [
                                cst_node(
                                    EXCLAMATION,
                                    [
                                        cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                                        cst_leaf("!"),
                                    ],
                                ),
                                // c
                                cst_node(
                                    ExprRepeat,
                                    [cst_node(
                                        ExprAtom,
                                        [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("c")])],
                                    )],
                                ),
                            ],
                        ),
                        // d
                        cst_node(
                            ExprPredicate,
                            [cst_node(
                                ExprRepeat,
                                [cst_node(
                                    ExprAtom,
                                    [cst_node(
                                        IDENT,
                                        [
                                            cst_node(
                                                Trivia,
                                                [cst_node(Whitespace, [cst_leaf(" ")])],
                                            ),
                                            cst_leaf("d"),
                                        ],
                                    )],
                                )],
                            )],
                        ),
                        // d*
                        cst_node(
                            ExprPredicate,
                            [cst_node(
                                ExprRepeat,
                                [
                                    cst_node(
                                        ExprAtom,
                                        [cst_node(
                                            IDENT,
                                            [
                                                cst_node(
                                                    Trivia,
                                                    [cst_node(Whitespace, [cst_leaf(" ")])],
                                                ),
                                                cst_leaf("d"),
                                            ],
                                        )],
                                    ),
                                    cst_node(
                                        RepeatOp,
                                        [cst_node(STAR, [cst_node(Trivia, []), cst_leaf("*")])],
                                    ),
                                ],
                            )],
                        ),
                        // (e / &f g)
                        cst_node(
                            ExprPredicate,
                            [cst_node(
                                ExprRepeat,
                                [cst_node(
                                    ExprAtom,
                                    [
                                        cst_node(
                                            PAREN_L,
                                            [
                                                cst_node(
                                                    Trivia,
                                                    [cst_node(Whitespace, [cst_leaf(" ")])],
                                                ),
                                                cst_leaf("("),
                                            ],
                                        ),
                                        // e / &f g
                                        cst_node(
                                            Expr,
                                            [cst_node(
                                                ExprChoice,
                                                [
                                                    // e
                                                    cst_node(
                                                        ExprSeq,
                                                        [cst_node(
                                                            ExprPredicate,
                                                            [cst_node(
                                                                ExprRepeat,
                                                                [cst_node(
                                                                    ExprAtom,
                                                                    [cst_node(
                                                                        IDENT,
                                                                        [
                                                                            cst_node(Trivia, []),
                                                                            cst_leaf("e"),
                                                                        ],
                                                                    )],
                                                                )],
                                                            )],
                                                        )],
                                                    ),
                                                    cst_node(
                                                        SLASH_F,
                                                        [
                                                            cst_node(
                                                                Trivia,
                                                                [cst_node(
                                                                    Whitespace,
                                                                    [cst_leaf(" ")],
                                                                )],
                                                            ),
                                                            cst_leaf("/"),
                                                        ],
                                                    ),
                                                    // &f g
                                                    cst_node(
                                                        ExprSeq,
                                                        [
                                                            // &f
                                                            cst_node(
                                                                ExprPredicate,
                                                                [
                                                                    cst_node(
                                                                        AMPERSAND,
                                                                        [
                                                                            cst_node(
                                                                                Trivia,
                                                                                [cst_node(
                                                                                    Whitespace,
                                                                                    [cst_leaf(" ")],
                                                                                )],
                                                                            ),
                                                                            cst_leaf("&"),
                                                                        ],
                                                                    ),
                                                                    // f
                                                                    cst_node(
                                                                        ExprRepeat,
                                                                        [cst_node(
                                                                            ExprAtom,
                                                                            [cst_node(
                                                                                IDENT,
                                                                                [
                                                                                    cst_node(
                                                                                        Trivia,
                                                                                        [],
                                                                                    ),
                                                                                    cst_leaf("f"),
                                                                                ],
                                                                            )],
                                                                        )],
                                                                    ),
                                                                ],
                                                            ),
                                                            // g
                                                            cst_node(
                                                                ExprPredicate,
                                                                [cst_node(
                                                                    ExprRepeat,
                                                                    [cst_node(
                                                                        ExprAtom,
                                                                        [cst_node(
                                                                            IDENT,
                                                                            [
                                                                                cst_node(
                                                                                    Trivia,
                                                                                    [cst_node(
                                                                                        Whitespace,
                                                                                        [cst_leaf(
                                                                                            " ",
                                                                                        )],
                                                                                    )],
                                                                                ),
                                                                                cst_leaf("g"),
                                                                            ],
                                                                        )],
                                                                    )],
                                                                )],
                                                            ),
                                                        ],
                                                    ),
                                                ],
                                            )],
                                        ),
                                        cst_node(PAREN_R, [cst_node(Trivia, []), cst_leaf(")")]),
                                    ],
                                )],
                            )],
                        ),
                    ],
                ),
            ],
        )],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_choice_atom() {
    let cst = parse_rule("uwu", ExprChoice);
    let expected = cst_node(
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
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_choice_two() {
    let cst = parse_rule("uwu/owo", ExprChoice);
    let expected = cst_node(
        ExprChoice,
        [
            cst_node(
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
            ),
            cst_node(SLASH_F, [cst_node(Trivia, []), cst_leaf("/")]),
            cst_node(
                ExprSeq,
                [cst_node(
                    ExprPredicate,
                    [cst_node(
                        ExprRepeat,
                        [cst_node(
                            ExprAtom,
                            [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("owo")])],
                        )],
                    )],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_choice_many() {
    let cst = parse_rule("uwu/owo/iwi/awa", ExprChoice);
    let expected = cst_node(
        ExprChoice,
        [
            cst_node(
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
            ),
            cst_node(SLASH_F, [cst_node(Trivia, []), cst_leaf("/")]),
            cst_node(
                ExprSeq,
                [cst_node(
                    ExprPredicate,
                    [cst_node(
                        ExprRepeat,
                        [cst_node(
                            ExprAtom,
                            [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("owo")])],
                        )],
                    )],
                )],
            ),
            cst_node(SLASH_F, [cst_node(Trivia, []), cst_leaf("/")]),
            cst_node(
                ExprSeq,
                [cst_node(
                    ExprPredicate,
                    [cst_node(
                        ExprRepeat,
                        [cst_node(
                            ExprAtom,
                            [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("iwi")])],
                        )],
                    )],
                )],
            ),
            cst_node(SLASH_F, [cst_node(Trivia, []), cst_leaf("/")]),
            cst_node(
                ExprSeq,
                [cst_node(
                    ExprPredicate,
                    [cst_node(
                        ExprRepeat,
                        [cst_node(
                            ExprAtom,
                            [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("awa")])],
                        )],
                    )],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_seq_atom() {
    let cst = parse_rule("uwu", ExprSeq);
    let expected = cst_node(
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
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_seq_two() {
    let cst = parse_rule("uwu owo", ExprSeq);
    let expected = cst_node(
        ExprSeq,
        [
            cst_node(
                ExprPredicate,
                [cst_node(
                    ExprRepeat,
                    [cst_node(
                        ExprAtom,
                        [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
                    )],
                )],
            ),
            cst_node(
                ExprPredicate,
                [cst_node(
                    ExprRepeat,
                    [cst_node(
                        ExprAtom,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                                cst_leaf("owo"),
                            ],
                        )],
                    )],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_seq_a_lot() {
    let cst = parse_rule("uwu owo iwi ewe awa", ExprSeq);
    let expected = cst_node(
        ExprSeq,
        [
            cst_node(
                ExprPredicate,
                [cst_node(
                    ExprRepeat,
                    [cst_node(
                        ExprAtom,
                        [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
                    )],
                )],
            ),
            cst_node(
                ExprPredicate,
                [cst_node(
                    ExprRepeat,
                    [cst_node(
                        ExprAtom,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                                cst_leaf("owo"),
                            ],
                        )],
                    )],
                )],
            ),
            cst_node(
                ExprPredicate,
                [cst_node(
                    ExprRepeat,
                    [cst_node(
                        ExprAtom,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                                cst_leaf("iwi"),
                            ],
                        )],
                    )],
                )],
            ),
            cst_node(
                ExprPredicate,
                [cst_node(
                    ExprRepeat,
                    [cst_node(
                        ExprAtom,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                                cst_leaf("ewe"),
                            ],
                        )],
                    )],
                )],
            ),
            cst_node(
                ExprPredicate,
                [cst_node(
                    ExprRepeat,
                    [cst_node(
                        ExprAtom,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(Trivia, [cst_node(Whitespace, [cst_leaf(" ")])]),
                                cst_leaf("awa"),
                            ],
                        )],
                    )],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_predicate_atom() {
    let cst = parse_rule("uwu", ExprPredicate);
    let expected = cst_node(
        ExprPredicate,
        [cst_node(
            ExprRepeat,
            [cst_node(
                ExprAtom,
                [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
            )],
        )],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_predicate_postive() {
    let cst = parse_rule("&uwu", ExprPredicate);
    let expected = cst_node(
        ExprPredicate,
        [
            cst_node(AMPERSAND, [cst_node(Trivia, []), cst_leaf("&")]),
            cst_node(
                ExprRepeat,
                [cst_node(
                    ExprAtom,
                    [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_predicate_negative() {
    let cst = parse_rule("!uwu", ExprPredicate);
    let expected = cst_node(
        ExprPredicate,
        [
            cst_node(EXCLAMATION, [cst_node(Trivia, []), cst_leaf("!")]),
            cst_node(
                ExprRepeat,
                [cst_node(
                    ExprAtom,
                    [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
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
fn expr_repeat_star() {
    let cst = parse_rule("uwu*", ExprRepeat);
    let expected = cst_node(
        ExprRepeat,
        [
            cst_node(
                ExprAtom,
                [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
            ),
            cst_node(
                RepeatOp,
                [cst_node(STAR, [cst_node(Trivia, []), cst_leaf("*")])],
            ),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_repeat_plus() {
    let cst = parse_rule("uwu+", ExprRepeat);
    let expected = cst_node(
        ExprRepeat,
        [
            cst_node(
                ExprAtom,
                [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
            ),
            cst_node(
                RepeatOp,
                [cst_node(PLUS, [cst_node(Trivia, []), cst_leaf("+")])],
            ),
        ],
    );
    pretty_assertions::assert_eq!(*cst, expected);
}

#[test]
fn expr_repeat_question() {
    let cst = parse_rule("uwu?", ExprRepeat);
    let expected = cst_node(
        ExprRepeat,
        [
            cst_node(
                ExprAtom,
                [cst_node(IDENT, [cst_node(Trivia, []), cst_leaf("uwu")])],
            ),
            cst_node(
                RepeatOp,
                [cst_node(QUESTION, [cst_node(Trivia, []), cst_leaf("?")])],
            ),
        ],
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
