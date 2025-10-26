use std::sync::Arc;

use pegme_core::cst::ConcreteSyntaxTree;

use crate::parser::{
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
    let cst = parse_rule("", FILE);
    let expected = cst_node(FILE, [cst_node(TRIVIA, []), cst_node(EOF, [])]);
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn file_two_rules() {
    let cst = parse_rule("// Hello\nrule One = a;\nrule Two = b;", FILE);
    let expected = cst_node(
        FILE,
        [
            cst_node(
                ITEM_RULE,
                [
                    cst_node(
                        RULE_KIND,
                        [cst_node(
                            RULE,
                            [
                                cst_node(
                                    TRIVIA,
                                    [cst_node(
                                        COMMENT,
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
                            cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                            cst_leaf("One"),
                        ],
                    ),
                    cst_node(
                        EQUAL,
                        [
                            cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                            cst_leaf("="),
                        ],
                    ),
                    cst_node(
                        EXPR,
                        [cst_node(
                            EXPR_CHOICE,
                            [cst_node(
                                EXPR_SEQ,
                                [cst_node(
                                    EXPR_PREDICATE,
                                    [cst_node(
                                        EXPR_REPEAT,
                                        [cst_node(
                                            EXPR_ATOM,
                                            [cst_node(
                                                IDENT,
                                                [
                                                    cst_node(
                                                        TRIVIA,
                                                        [cst_node(WHITESPACE, [cst_leaf(" ")])],
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
                    cst_node(SEMICOLON, [cst_node(TRIVIA, []), cst_leaf(";")]),
                ],
            ),
            cst_node(
                ITEM_RULE,
                [
                    cst_node(
                        RULE_KIND,
                        [cst_node(
                            RULE,
                            [
                                cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf("\n")])]),
                                cst_leaf("rule"),
                            ],
                        )],
                    ),
                    cst_node(
                        IDENT,
                        [
                            cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                            cst_leaf("Two"),
                        ],
                    ),
                    cst_node(
                        EQUAL,
                        [
                            cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                            cst_leaf("="),
                        ],
                    ),
                    cst_node(
                        EXPR,
                        [cst_node(
                            EXPR_CHOICE,
                            [cst_node(
                                EXPR_SEQ,
                                [cst_node(
                                    EXPR_PREDICATE,
                                    [cst_node(
                                        EXPR_REPEAT,
                                        [cst_node(
                                            EXPR_ATOM,
                                            [cst_node(
                                                IDENT,
                                                [
                                                    cst_node(
                                                        TRIVIA,
                                                        [cst_node(WHITESPACE, [cst_leaf(" ")])],
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
                    cst_node(SEMICOLON, [cst_node(TRIVIA, []), cst_leaf(";")]),
                ],
            ),
            cst_node(TRIVIA, []),
            cst_node(EOF, []),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn rule_simple() {
    let cst = parse_rule("rule hello = hello;", ITEM_RULE);
    let expected = cst_node(
        ITEM_RULE,
        [
            cst_node(
                RULE_KIND,
                [cst_node(RULE, [cst_node(TRIVIA, []), cst_leaf("rule")])],
            ),
            cst_node(
                IDENT,
                [
                    cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                    cst_leaf("hello"),
                ],
            ),
            cst_node(
                EQUAL,
                [
                    cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                    cst_leaf("="),
                ],
            ),
            cst_node(
                EXPR,
                [cst_node(
                    EXPR_CHOICE,
                    [cst_node(
                        EXPR_SEQ,
                        [cst_node(
                            EXPR_PREDICATE,
                            [cst_node(
                                EXPR_REPEAT,
                                [cst_node(
                                    EXPR_ATOM,
                                    [cst_node(
                                        IDENT,
                                        [
                                            cst_node(
                                                TRIVIA,
                                                [cst_node(WHITESPACE, [cst_leaf(" ")])],
                                            ),
                                            cst_leaf("hello"),
                                        ],
                                    )],
                                )],
                            )],
                        )],
                    )],
                )],
            ),
            cst_node(SEMICOLON, [cst_node(TRIVIA, []), cst_leaf(";")]),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_atom() {
    let cst = parse_rule("uwu", EXPR);
    let expected = cst_node(
        EXPR,
        [cst_node(
            EXPR_CHOICE,
            [cst_node(
                EXPR_SEQ,
                [cst_node(
                    EXPR_PREDICATE,
                    [cst_node(
                        EXPR_REPEAT,
                        [cst_node(
                            EXPR_ATOM,
                            [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                        )],
                    )],
                )],
            )],
        )],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_bullshit() {
    let cst = parse_rule("a b / !c d d* (e / &f g)", EXPR);
    let expected = cst_node(
        EXPR,
        [cst_node(
            EXPR_CHOICE,
            [
                // a b
                cst_node(
                    EXPR_SEQ,
                    [
                        // a
                        cst_node(
                            EXPR_PREDICATE,
                            [cst_node(
                                EXPR_REPEAT,
                                [cst_node(
                                    EXPR_ATOM,
                                    [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("a")])],
                                )],
                            )],
                        ),
                        // b
                        cst_node(
                            EXPR_PREDICATE,
                            [cst_node(
                                EXPR_REPEAT,
                                [cst_node(
                                    EXPR_ATOM,
                                    [cst_node(
                                        IDENT,
                                        [
                                            cst_node(
                                                TRIVIA,
                                                [cst_node(WHITESPACE, [cst_leaf(" ")])],
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
                        cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                        cst_leaf("/"),
                    ],
                ),
                cst_node(
                    EXPR_SEQ,
                    [
                        // !c
                        cst_node(
                            EXPR_PREDICATE,
                            [
                                cst_node(
                                    PREDICATE_OP,
                                    [cst_node(
                                        BANG,
                                        [
                                            cst_node(
                                                TRIVIA,
                                                [cst_node(WHITESPACE, [cst_leaf(" ")])],
                                            ),
                                            cst_leaf("!"),
                                        ],
                                    )],
                                ),
                                // c
                                cst_node(
                                    EXPR_REPEAT,
                                    [cst_node(
                                        EXPR_ATOM,
                                        [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("c")])],
                                    )],
                                ),
                            ],
                        ),
                        // d
                        cst_node(
                            EXPR_PREDICATE,
                            [cst_node(
                                EXPR_REPEAT,
                                [cst_node(
                                    EXPR_ATOM,
                                    [cst_node(
                                        IDENT,
                                        [
                                            cst_node(
                                                TRIVIA,
                                                [cst_node(WHITESPACE, [cst_leaf(" ")])],
                                            ),
                                            cst_leaf("d"),
                                        ],
                                    )],
                                )],
                            )],
                        ),
                        // d*
                        cst_node(
                            EXPR_PREDICATE,
                            [cst_node(
                                EXPR_REPEAT,
                                [
                                    cst_node(
                                        EXPR_ATOM,
                                        [cst_node(
                                            IDENT,
                                            [
                                                cst_node(
                                                    TRIVIA,
                                                    [cst_node(WHITESPACE, [cst_leaf(" ")])],
                                                ),
                                                cst_leaf("d"),
                                            ],
                                        )],
                                    ),
                                    cst_node(
                                        REPEAT_OP,
                                        [cst_node(STAR, [cst_node(TRIVIA, []), cst_leaf("*")])],
                                    ),
                                ],
                            )],
                        ),
                        // (e / &f g)
                        cst_node(
                            EXPR_PREDICATE,
                            [cst_node(
                                EXPR_REPEAT,
                                [cst_node(
                                    EXPR_ATOM,
                                    [
                                        cst_node(
                                            PAREN_L,
                                            [
                                                cst_node(
                                                    TRIVIA,
                                                    [cst_node(WHITESPACE, [cst_leaf(" ")])],
                                                ),
                                                cst_leaf("("),
                                            ],
                                        ),
                                        // e / &f g
                                        cst_node(
                                            EXPR,
                                            [cst_node(
                                                EXPR_CHOICE,
                                                [
                                                    // e
                                                    cst_node(
                                                        EXPR_SEQ,
                                                        [cst_node(
                                                            EXPR_PREDICATE,
                                                            [cst_node(
                                                                EXPR_REPEAT,
                                                                [cst_node(
                                                                    EXPR_ATOM,
                                                                    [cst_node(
                                                                        IDENT,
                                                                        [
                                                                            cst_node(TRIVIA, []),
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
                                                                TRIVIA,
                                                                [cst_node(
                                                                    WHITESPACE,
                                                                    [cst_leaf(" ")],
                                                                )],
                                                            ),
                                                            cst_leaf("/"),
                                                        ],
                                                    ),
                                                    // &f g
                                                    cst_node(
                                                        EXPR_SEQ,
                                                        [
                                                            // &f
                                                            cst_node(
                                                                EXPR_PREDICATE,
                                                                [
                                                                    cst_node(
                                                                        PREDICATE_OP,
                                                                        [cst_node(
                                                                            AND,
                                                                            [
                                                                                cst_node(
                                                                                    TRIVIA,
                                                                                    [cst_node(
                                                                                        WHITESPACE,
                                                                                        [cst_leaf(
                                                                                            " ",
                                                                                        )],
                                                                                    )],
                                                                                ),
                                                                                cst_leaf("&"),
                                                                            ],
                                                                        )],
                                                                    ),
                                                                    // f
                                                                    cst_node(
                                                                        EXPR_REPEAT,
                                                                        [cst_node(
                                                                            EXPR_ATOM,
                                                                            [cst_node(
                                                                                IDENT,
                                                                                [
                                                                                    cst_node(
                                                                                        TRIVIA,
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
                                                                EXPR_PREDICATE,
                                                                [cst_node(
                                                                    EXPR_REPEAT,
                                                                    [cst_node(
                                                                        EXPR_ATOM,
                                                                        [cst_node(
                                                                            IDENT,
                                                                            [
                                                                                cst_node(
                                                                                    TRIVIA,
                                                                                    [cst_node(
                                                                                        WHITESPACE,
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
                                        cst_node(PAREN_R, [cst_node(TRIVIA, []), cst_leaf(")")]),
                                    ],
                                )],
                            )],
                        ),
                    ],
                ),
            ],
        )],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_choice_atom() {
    let cst = parse_rule("uwu", EXPR_CHOICE);
    let expected = cst_node(
        EXPR_CHOICE,
        [cst_node(
            EXPR_SEQ,
            [cst_node(
                EXPR_PREDICATE,
                [cst_node(
                    EXPR_REPEAT,
                    [cst_node(
                        EXPR_ATOM,
                        [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                    )],
                )],
            )],
        )],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_choice_two() {
    let cst = parse_rule("uwu/owo", EXPR_CHOICE);
    let expected = cst_node(
        EXPR_CHOICE,
        [
            cst_node(
                EXPR_SEQ,
                [cst_node(
                    EXPR_PREDICATE,
                    [cst_node(
                        EXPR_REPEAT,
                        [cst_node(
                            EXPR_ATOM,
                            [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                        )],
                    )],
                )],
            ),
            cst_node(SLASH_F, [cst_node(TRIVIA, []), cst_leaf("/")]),
            cst_node(
                EXPR_SEQ,
                [cst_node(
                    EXPR_PREDICATE,
                    [cst_node(
                        EXPR_REPEAT,
                        [cst_node(
                            EXPR_ATOM,
                            [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("owo")])],
                        )],
                    )],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_choice_many() {
    let cst = parse_rule("uwu/owo/iwi/awa", EXPR_CHOICE);
    let expected = cst_node(
        EXPR_CHOICE,
        [
            cst_node(
                EXPR_SEQ,
                [cst_node(
                    EXPR_PREDICATE,
                    [cst_node(
                        EXPR_REPEAT,
                        [cst_node(
                            EXPR_ATOM,
                            [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                        )],
                    )],
                )],
            ),
            cst_node(SLASH_F, [cst_node(TRIVIA, []), cst_leaf("/")]),
            cst_node(
                EXPR_SEQ,
                [cst_node(
                    EXPR_PREDICATE,
                    [cst_node(
                        EXPR_REPEAT,
                        [cst_node(
                            EXPR_ATOM,
                            [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("owo")])],
                        )],
                    )],
                )],
            ),
            cst_node(SLASH_F, [cst_node(TRIVIA, []), cst_leaf("/")]),
            cst_node(
                EXPR_SEQ,
                [cst_node(
                    EXPR_PREDICATE,
                    [cst_node(
                        EXPR_REPEAT,
                        [cst_node(
                            EXPR_ATOM,
                            [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("iwi")])],
                        )],
                    )],
                )],
            ),
            cst_node(SLASH_F, [cst_node(TRIVIA, []), cst_leaf("/")]),
            cst_node(
                EXPR_SEQ,
                [cst_node(
                    EXPR_PREDICATE,
                    [cst_node(
                        EXPR_REPEAT,
                        [cst_node(
                            EXPR_ATOM,
                            [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("awa")])],
                        )],
                    )],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_seq_atom() {
    let cst = parse_rule("uwu", EXPR_SEQ);
    let expected = cst_node(
        EXPR_SEQ,
        [cst_node(
            EXPR_PREDICATE,
            [cst_node(
                EXPR_REPEAT,
                [cst_node(
                    EXPR_ATOM,
                    [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                )],
            )],
        )],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_seq_two() {
    let cst = parse_rule("uwu owo", EXPR_SEQ);
    let expected = cst_node(
        EXPR_SEQ,
        [
            cst_node(
                EXPR_PREDICATE,
                [cst_node(
                    EXPR_REPEAT,
                    [cst_node(
                        EXPR_ATOM,
                        [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                    )],
                )],
            ),
            cst_node(
                EXPR_PREDICATE,
                [cst_node(
                    EXPR_REPEAT,
                    [cst_node(
                        EXPR_ATOM,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                                cst_leaf("owo"),
                            ],
                        )],
                    )],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_seq_a_lot() {
    let cst = parse_rule("uwu owo iwi ewe awa", EXPR_SEQ);
    let expected = cst_node(
        EXPR_SEQ,
        [
            cst_node(
                EXPR_PREDICATE,
                [cst_node(
                    EXPR_REPEAT,
                    [cst_node(
                        EXPR_ATOM,
                        [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                    )],
                )],
            ),
            cst_node(
                EXPR_PREDICATE,
                [cst_node(
                    EXPR_REPEAT,
                    [cst_node(
                        EXPR_ATOM,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                                cst_leaf("owo"),
                            ],
                        )],
                    )],
                )],
            ),
            cst_node(
                EXPR_PREDICATE,
                [cst_node(
                    EXPR_REPEAT,
                    [cst_node(
                        EXPR_ATOM,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                                cst_leaf("iwi"),
                            ],
                        )],
                    )],
                )],
            ),
            cst_node(
                EXPR_PREDICATE,
                [cst_node(
                    EXPR_REPEAT,
                    [cst_node(
                        EXPR_ATOM,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                                cst_leaf("ewe"),
                            ],
                        )],
                    )],
                )],
            ),
            cst_node(
                EXPR_PREDICATE,
                [cst_node(
                    EXPR_REPEAT,
                    [cst_node(
                        EXPR_ATOM,
                        [cst_node(
                            IDENT,
                            [
                                cst_node(TRIVIA, [cst_node(WHITESPACE, [cst_leaf(" ")])]),
                                cst_leaf("awa"),
                            ],
                        )],
                    )],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_predicate_atom() {
    let cst = parse_rule("uwu", EXPR_PREDICATE);
    let expected = cst_node(
        EXPR_PREDICATE,
        [cst_node(
            EXPR_REPEAT,
            [cst_node(
                EXPR_ATOM,
                [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
            )],
        )],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_predicate_postive() {
    let cst = parse_rule("&uwu", EXPR_PREDICATE);
    let expected = cst_node(
        EXPR_PREDICATE,
        [
            cst_node(
                PREDICATE_OP,
                [cst_node(AND, [cst_node(TRIVIA, []), cst_leaf("&")])],
            ),
            cst_node(
                EXPR_REPEAT,
                [cst_node(
                    EXPR_ATOM,
                    [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_predicate_negative() {
    let cst = parse_rule("!uwu", EXPR_PREDICATE);
    let expected = cst_node(
        EXPR_PREDICATE,
        [
            cst_node(
                PREDICATE_OP,
                [cst_node(BANG, [cst_node(TRIVIA, []), cst_leaf("!")])],
            ),
            cst_node(
                EXPR_REPEAT,
                [cst_node(
                    EXPR_ATOM,
                    [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                )],
            ),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_repeat_atom() {
    let cst = parse_rule("uwu", EXPR_REPEAT);
    let expected = cst_node(
        EXPR_REPEAT,
        [cst_node(
            EXPR_ATOM,
            [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
        )],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_repeat_star() {
    let cst = parse_rule("uwu*", EXPR_REPEAT);
    let expected = cst_node(
        EXPR_REPEAT,
        [
            cst_node(
                EXPR_ATOM,
                [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
            ),
            cst_node(
                REPEAT_OP,
                [cst_node(STAR, [cst_node(TRIVIA, []), cst_leaf("*")])],
            ),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_repeat_plus() {
    let cst = parse_rule("uwu+", EXPR_REPEAT);
    let expected = cst_node(
        EXPR_REPEAT,
        [
            cst_node(
                EXPR_ATOM,
                [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
            ),
            cst_node(
                REPEAT_OP,
                [cst_node(PLUS, [cst_node(TRIVIA, []), cst_leaf("+")])],
            ),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_repeat_question() {
    let cst = parse_rule("uwu?", EXPR_REPEAT);
    let expected = cst_node(
        EXPR_REPEAT,
        [
            cst_node(
                EXPR_ATOM,
                [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
            ),
            cst_node(
                REPEAT_OP,
                [cst_node(QUESTION, [cst_node(TRIVIA, []), cst_leaf("?")])],
            ),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_atom_paren() {
    let cst = parse_rule("(uwu)", EXPR_ATOM);
    let expected = cst_node(
        EXPR_ATOM,
        [
            cst_node(PAREN_L, [cst_node(TRIVIA, []), cst_leaf("(")]),
            cst_node(
                EXPR,
                [cst_node(
                    EXPR_CHOICE,
                    [cst_node(
                        EXPR_SEQ,
                        [cst_node(
                            EXPR_PREDICATE,
                            [cst_node(
                                EXPR_REPEAT,
                                [cst_node(
                                    EXPR_ATOM,
                                    [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("uwu")])],
                                )],
                            )],
                        )],
                    )],
                )],
            ),
            cst_node(PAREN_R, [cst_node(TRIVIA, []), cst_leaf(")")]),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_atom_ident() {
    let cst = parse_rule("hello", EXPR_ATOM);
    let expected = cst_node(
        EXPR_ATOM,
        [cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("hello")])],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_atom_dot() {
    let cst = parse_rule(".", EXPR_ATOM);
    let expected = cst_node(
        EXPR_ATOM,
        [cst_node(DOT, [cst_node(TRIVIA, []), cst_leaf(".")])],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn expr_atom_keyword() {
    let cst = parse_rule("\"uwu\"", EXPR_ATOM);
    let expected = cst_node(
        EXPR_ATOM,
        [cst_node(
            LITERAL,
            [cst_node(TRIVIA, []), cst_leaf("\"uwu\"")],
        )],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn trivia() {
    let cst = parse_rule("  \t\n \r\n // hello \n", TRIVIA);

    let expected = cst_node(
        TRIVIA,
        [
            cst_node(WHITESPACE, [cst_leaf("  \t\n \r\n ")]),
            cst_node(
                COMMENT,
                [cst_leaf("// hello "), cst_node(EOL, [cst_leaf("\n")])],
            ),
        ],
    );

    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn keyword_simple() {
    let cst = parse_rule("\"hello\"", LITERAL);
    let expected = cst_node(LITERAL, [cst_node(TRIVIA, []), cst_leaf("\"hello\"")]);
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn keyword_escaped() {
    let cst = parse_rule("\"he\\\"llo\"", LITERAL);
    let expected = cst_node(LITERAL, [cst_node(TRIVIA, []), cst_leaf("\"he\\\"llo\"")]);
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn character_ranges_simple() {
    let cst = parse_rule("[a]", RANGES);
    let expected = cst_node(
        RANGES,
        [
            cst_node(TRIVIA, []),
            cst_leaf("["),
            cst_node(RANGE_SOLO, [cst_leaf("a")]),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn character_ranges_varied() {
    let cst = parse_rule("[a0_'\"]", RANGES);
    let expected = cst_node(
        RANGES,
        [
            cst_node(TRIVIA, []),
            cst_leaf("["),
            cst_node(RANGE_SOLO, [cst_leaf("a")]),
            cst_node(RANGE_SOLO, [cst_leaf("0")]),
            cst_node(RANGE_SOLO, [cst_leaf("_")]),
            cst_node(RANGE_SOLO, [cst_leaf("'")]),
            cst_node(RANGE_SOLO, [cst_leaf("\"")]),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn character_ranges_escaped() {
    let cst = parse_rule("[\\]]", RANGES);
    let expected = cst_node(
        RANGES,
        [
            cst_node(TRIVIA, []),
            cst_leaf("["),
            cst_node(RANGE_SOLO, [cst_leaf("\\]")]),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn character_ranges_escaped_range() {
    let cst = parse_rule("[\\[-\\]]", RANGES);
    let expected = cst_node(
        RANGES,
        [
            cst_node(TRIVIA, []),
            cst_leaf("["),
            cst_node(
                RANGE_RANGE,
                [
                    cst_node(RANGE_SOLO, [cst_leaf("\\[")]),
                    cst_leaf("-"),
                    cst_node(RANGE_SOLO, [cst_leaf("\\]")]),
                ],
            ),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn character_ranges_single_range() {
    let cst = parse_rule("[a-z]", RANGES);
    let expected = cst_node(
        RANGES,
        [
            cst_node(TRIVIA, []),
            cst_leaf("["),
            cst_node(
                RANGE_RANGE,
                [
                    cst_node(RANGE_SOLO, [cst_leaf("a")]),
                    cst_leaf("-"),
                    cst_node(RANGE_SOLO, [cst_leaf("z")]),
                ],
            ),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn character_ranges_multiple_range() {
    let cst = parse_rule("[a-z0-9]", RANGES);
    let expected = cst_node(
        RANGES,
        [
            cst_node(TRIVIA, []),
            cst_leaf("["),
            cst_node(
                RANGE_RANGE,
                [
                    cst_node(RANGE_SOLO, [cst_leaf("a")]),
                    cst_leaf("-"),
                    cst_node(RANGE_SOLO, [cst_leaf("z")]),
                ],
            ),
            cst_node(
                RANGE_RANGE,
                [
                    cst_node(RANGE_SOLO, [cst_leaf("0")]),
                    cst_leaf("-"),
                    cst_node(RANGE_SOLO, [cst_leaf("9")]),
                ],
            ),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn character_ranges_mixed() {
    let cst = parse_rule("[a-z012B-G]", RANGES);
    let expected = cst_node(
        RANGES,
        [
            cst_node(TRIVIA, []),
            cst_leaf("["),
            cst_node(
                RANGE_RANGE,
                [
                    cst_node(RANGE_SOLO, [cst_leaf("a")]),
                    cst_leaf("-"),
                    cst_node(RANGE_SOLO, [cst_leaf("z")]),
                ],
            ),
            cst_node(RANGE_SOLO, [cst_leaf("0")]),
            cst_node(RANGE_SOLO, [cst_leaf("1")]),
            cst_node(RANGE_SOLO, [cst_leaf("2")]),
            cst_node(
                RANGE_RANGE,
                [
                    cst_node(RANGE_SOLO, [cst_leaf("B")]),
                    cst_leaf("-"),
                    cst_node(RANGE_SOLO, [cst_leaf("G")]),
                ],
            ),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn character_ranges_dashdashdashdash() {
    let cst = parse_rule("[----]", RANGES);
    let expected = cst_node(
        RANGES,
        [
            cst_node(TRIVIA, []),
            cst_leaf("["),
            cst_node(
                RANGE_RANGE,
                [
                    cst_node(RANGE_SOLO, [cst_leaf("-")]),
                    cst_leaf("-"),
                    cst_node(RANGE_SOLO, [cst_leaf("-")]),
                ],
            ),
            cst_node(RANGE_SOLO, [cst_leaf("-")]),
            cst_leaf("]"),
        ],
    );
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn ident_alphabetic() {
    let cst = parse_rule("hello", IDENT);
    let expected = cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("hello")]);
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn ident_alphanumeric() {
    let cst = parse_rule("a123", IDENT);
    let expected = cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("a123")]);
    pretty_assertions::assert_eq!(expected, *cst);
}

#[test]
fn ident_bs() {
    let cst = parse_rule("_a1_23", IDENT);
    let expected = cst_node(IDENT, [cst_node(TRIVIA, []), cst_leaf("_a1_23")]);
    pretty_assertions::assert_eq!(expected, *cst);
}
