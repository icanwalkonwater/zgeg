#![doc = r" DO NOT EDIT."]
#![doc = r" This file is auto-generated."]
use crate::{
    cst::{ConcreteSyntaxTree, ConcreteSyntaxTreeBuilder},
    packrat::PackratParser,
};
use std::sync::Arc;
pub fn parse(input: impl Into<String>) -> Arc<ConcreteSyntaxTree<MetaPegmeKind>> {
    parse_rule(input, MetaPegmeKind::File)
}
#[inline]
pub(crate) fn parse_rule(
    input: impl Into<String>,
    rule: MetaPegmeKind,
) -> Arc<ConcreteSyntaxTree<MetaPegmeKind>> {
    let mut parser_state = MetaPegmeParser {
        parser: PackratParser::new(input),
        tree: ConcreteSyntaxTreeBuilder::default(),
    };
    match rule {
        MetaPegmeKind::File => {
            let valid = parser_state.test_File();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::File);
            parser_state.parser.reset();
            parser_state.parse_File();
            parser_state.tree.build()
        }
        MetaPegmeKind::Rule => {
            let valid = parser_state.test_Rule();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::Rule);
            parser_state.parser.reset();
            parser_state.parse_Rule();
            parser_state.tree.build()
        }
        MetaPegmeKind::RuleKind => {
            let valid = parser_state.test_RuleKind();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::RuleKind);
            parser_state.parser.reset();
            parser_state.parse_RuleKind();
            parser_state.tree.build()
        }
        MetaPegmeKind::Expr => {
            let valid = parser_state.test_Expr();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::Expr);
            parser_state.parser.reset();
            parser_state.parse_Expr();
            parser_state.tree.build()
        }
        MetaPegmeKind::ExprChoice => {
            let valid = parser_state.test_ExprChoice();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::ExprChoice);
            parser_state.parser.reset();
            parser_state.parse_ExprChoice();
            parser_state.tree.build()
        }
        MetaPegmeKind::ExprSeq => {
            let valid = parser_state.test_ExprSeq();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::ExprSeq);
            parser_state.parser.reset();
            parser_state.parse_ExprSeq();
            parser_state.tree.build()
        }
        MetaPegmeKind::ExprPredicate => {
            let valid = parser_state.test_ExprPredicate();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::ExprPredicate);
            parser_state.parser.reset();
            parser_state.parse_ExprPredicate();
            parser_state.tree.build()
        }
        MetaPegmeKind::ExprRepeat => {
            let valid = parser_state.test_ExprRepeat();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::ExprRepeat);
            parser_state.parser.reset();
            parser_state.parse_ExprRepeat();
            parser_state.tree.build()
        }
        MetaPegmeKind::ExprAtom => {
            let valid = parser_state.test_ExprAtom();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::ExprAtom);
            parser_state.parser.reset();
            parser_state.parse_ExprAtom();
            parser_state.tree.build()
        }
        MetaPegmeKind::RepeatOp => {
            let valid = parser_state.test_RepeatOp();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::RepeatOp);
            parser_state.parser.reset();
            parser_state.parse_RepeatOp();
            parser_state.tree.build()
        }
        MetaPegmeKind::Keyword => {
            let valid = parser_state.test_Keyword();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::Keyword);
            parser_state.parser.reset();
            parser_state.parse_Keyword();
            parser_state.tree.build()
        }
        MetaPegmeKind::CharacterRanges => {
            let valid = parser_state.test_CharacterRanges();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::CharacterRanges);
            parser_state.parser.reset();
            parser_state.parse_CharacterRanges();
            parser_state.tree.build()
        }
        MetaPegmeKind::CharacterRangesIdent => {
            let valid = parser_state.test_CharacterRangesIdent();
            assert!(
                valid,
                "Couldn't parse {}",
                MetaPegmeKind::CharacterRangesIdent
            );
            parser_state.parser.reset();
            parser_state.parse_CharacterRangesIdent();
            parser_state.tree.build()
        }
        MetaPegmeKind::CharacterRangesRange => {
            let valid = parser_state.test_CharacterRangesRange();
            assert!(
                valid,
                "Couldn't parse {}",
                MetaPegmeKind::CharacterRangesRange
            );
            parser_state.parser.reset();
            parser_state.parse_CharacterRangesRange();
            parser_state.tree.build()
        }
        MetaPegmeKind::Whitespace => {
            let valid = parser_state.test_Whitespace();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::Whitespace);
            parser_state.parser.reset();
            parser_state.parse_Whitespace();
            parser_state.tree.build()
        }
        MetaPegmeKind::Comment => {
            let valid = parser_state.test_Comment();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::Comment);
            parser_state.parser.reset();
            parser_state.parse_Comment();
            parser_state.tree.build()
        }
        MetaPegmeKind::Trivia => {
            let valid = parser_state.test_Trivia();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::Trivia);
            parser_state.parser.reset();
            parser_state.parse_Trivia();
            parser_state.tree.build()
        }
        MetaPegmeKind::EOL => {
            let valid = parser_state.test_EOL();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::EOL);
            parser_state.parser.reset();
            parser_state.parse_EOL();
            parser_state.tree.build()
        }
        MetaPegmeKind::EOF => {
            let valid = parser_state.test_EOF();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::EOF);
            parser_state.parser.reset();
            parser_state.parse_EOF();
            parser_state.tree.build()
        }
        MetaPegmeKind::EOKW => {
            let valid = parser_state.test_EOKW();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::EOKW);
            parser_state.parser.reset();
            parser_state.parse_EOKW();
            parser_state.tree.build()
        }
        MetaPegmeKind::IDENT => {
            let valid = parser_state.test_IDENT();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::IDENT);
            parser_state.parser.reset();
            parser_state.parse_IDENT();
            parser_state.tree.build()
        }
        MetaPegmeKind::KW => {
            let valid = parser_state.test_KW();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::KW);
            parser_state.parser.reset();
            parser_state.parse_KW();
            parser_state.tree.build()
        }
        MetaPegmeKind::INTEGER => {
            let valid = parser_state.test_INTEGER();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::INTEGER);
            parser_state.parser.reset();
            parser_state.parse_INTEGER();
            parser_state.tree.build()
        }
        MetaPegmeKind::RULE => {
            let valid = parser_state.test_RULE();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::RULE);
            parser_state.parser.reset();
            parser_state.parse_RULE();
            parser_state.tree.build()
        }
        MetaPegmeKind::EQUAL => {
            let valid = parser_state.test_EQUAL();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::EQUAL);
            parser_state.parser.reset();
            parser_state.parse_EQUAL();
            parser_state.tree.build()
        }
        MetaPegmeKind::SLASH_F => {
            let valid = parser_state.test_SLASH_F();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::SLASH_F);
            parser_state.parser.reset();
            parser_state.parse_SLASH_F();
            parser_state.tree.build()
        }
        MetaPegmeKind::AMPERSAND => {
            let valid = parser_state.test_AMPERSAND();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::AMPERSAND);
            parser_state.parser.reset();
            parser_state.parse_AMPERSAND();
            parser_state.tree.build()
        }
        MetaPegmeKind::EXCLAMATION => {
            let valid = parser_state.test_EXCLAMATION();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::EXCLAMATION);
            parser_state.parser.reset();
            parser_state.parse_EXCLAMATION();
            parser_state.tree.build()
        }
        MetaPegmeKind::STAR => {
            let valid = parser_state.test_STAR();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::STAR);
            parser_state.parser.reset();
            parser_state.parse_STAR();
            parser_state.tree.build()
        }
        MetaPegmeKind::PLUS => {
            let valid = parser_state.test_PLUS();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::PLUS);
            parser_state.parser.reset();
            parser_state.parse_PLUS();
            parser_state.tree.build()
        }
        MetaPegmeKind::QUESTION => {
            let valid = parser_state.test_QUESTION();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::QUESTION);
            parser_state.parser.reset();
            parser_state.parse_QUESTION();
            parser_state.tree.build()
        }
        MetaPegmeKind::DOT => {
            let valid = parser_state.test_DOT();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::DOT);
            parser_state.parser.reset();
            parser_state.parse_DOT();
            parser_state.tree.build()
        }
        MetaPegmeKind::COLON => {
            let valid = parser_state.test_COLON();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::COLON);
            parser_state.parser.reset();
            parser_state.parse_COLON();
            parser_state.tree.build()
        }
        MetaPegmeKind::SEMICOLON => {
            let valid = parser_state.test_SEMICOLON();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::SEMICOLON);
            parser_state.parser.reset();
            parser_state.parse_SEMICOLON();
            parser_state.tree.build()
        }
        MetaPegmeKind::PAREN_L => {
            let valid = parser_state.test_PAREN_L();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::PAREN_L);
            parser_state.parser.reset();
            parser_state.parse_PAREN_L();
            parser_state.tree.build()
        }
        MetaPegmeKind::PAREN_R => {
            let valid = parser_state.test_PAREN_R();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::PAREN_R);
            parser_state.parser.reset();
            parser_state.parse_PAREN_R();
            parser_state.tree.build()
        }
        MetaPegmeKind::BRACES_L => {
            let valid = parser_state.test_BRACES_L();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::BRACES_L);
            parser_state.parser.reset();
            parser_state.parse_BRACES_L();
            parser_state.tree.build()
        }
        MetaPegmeKind::BRACES_R => {
            let valid = parser_state.test_BRACES_R();
            assert!(valid, "Couldn't parse {}", MetaPegmeKind::BRACES_R);
            parser_state.parser.reset();
            parser_state.parse_BRACES_R();
            parser_state.tree.build()
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaPegmeKind {
    File,
    Rule,
    RuleKind,
    Expr,
    ExprChoice,
    ExprSeq,
    ExprPredicate,
    ExprRepeat,
    ExprAtom,
    RepeatOp,
    Keyword,
    CharacterRanges,
    CharacterRangesIdent,
    CharacterRangesRange,
    Whitespace,
    Comment,
    Trivia,
    EOL,
    EOF,
    EOKW,
    IDENT,
    KW,
    INTEGER,
    RULE,
    EQUAL,
    SLASH_F,
    AMPERSAND,
    EXCLAMATION,
    STAR,
    PLUS,
    QUESTION,
    DOT,
    COLON,
    SEMICOLON,
    PAREN_L,
    PAREN_R,
    BRACES_L,
    BRACES_R,
}
impl std::fmt::Display for MetaPegmeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
pub(crate) struct MetaPegmeParser {
    pub(crate) parser: PackratParser<MetaPegmeKind>,
    pub(crate) tree: ConcreteSyntaxTreeBuilder<MetaPegmeKind>,
}
#[allow(non_snake_case)]
impl MetaPegmeParser {
    #[doc = "Rule* Trivia EOF"]
    pub(crate) fn parse_File(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::File, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::File);
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    match self.test_Rule() {
                        true => true,
                        false => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.parse_Rule();
        }
        self.parse_Trivia();
        self.parse_EOF();
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Rule* Trivia EOF"]
    pub(crate) fn test_File(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::File, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                while {
                    match self.test_Rule() {
                        true => true,
                        false => false,
                    }
                } {}
                true
            } {
                true => match {
                    match self.test_Trivia() {
                        true => true,
                        false => false,
                    }
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => match {
                match self.test_EOF() {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "File", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::File, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::File, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::File, start, self.parser.mark());
        true
    }
    #[doc = "RuleKind IDENT EQUAL Expr SEMICOLON"]
    pub(crate) fn parse_Rule(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::Rule, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::Rule);
        self.parse_RuleKind();
        self.parse_IDENT();
        self.parse_EQUAL();
        self.parse_Expr();
        self.parse_SEMICOLON();
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "RuleKind IDENT EQUAL Expr SEMICOLON"]
    pub(crate) fn test_Rule(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::Rule, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                let before_left = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        match self.test_RuleKind() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => match {
                            match self.test_IDENT() {
                                true => true,
                                false => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                false
                            }
                        },
                        false => false,
                    }
                } {
                    true => match {
                        match self.test_EQUAL() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            false
                        }
                    },
                    false => false,
                }
            } {
                true => match {
                    match self.test_Expr() {
                        true => true,
                        false => false,
                    }
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => match {
                match self.test_SEMICOLON() {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "Rule", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::Rule, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::Rule, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::Rule, start, self.parser.mark());
        true
    }
    #[doc = "RULE"]
    pub(crate) fn parse_RuleKind(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::RuleKind, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::RuleKind);
        self.parse_RULE();
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "RULE"]
    pub(crate) fn test_RuleKind(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::RuleKind, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        match self.test_RULE() {
            true => {
                tracing::trace!("Recognized rule {} at {:?}", "RuleKind", self.parser.mark());
            }
            false => {
                self.parser.memoize_miss(MetaPegmeKind::RuleKind, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::RuleKind, start, self.parser.mark());
        true
    }
    #[doc = "ExprChoice"]
    pub(crate) fn parse_Expr(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::Expr, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::Expr);
        self.parse_ExprChoice();
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "ExprChoice"]
    pub(crate) fn test_Expr(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::Expr, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        match self.test_ExprChoice() {
            true => {
                tracing::trace!("Recognized rule {} at {:?}", "Expr", self.parser.mark());
            }
            false => {
                self.parser.memoize_miss(MetaPegmeKind::Expr, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::Expr, start, self.parser.mark());
        true
    }
    #[doc = "ExprSeq (SLASH_F ExprSeq)*"]
    pub(crate) fn parse_ExprChoice(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::ExprChoice, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::ExprChoice);
        self.parse_ExprSeq();
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        match self.test_SLASH_F() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => match {
                            match self.test_ExprSeq() {
                                true => true,
                                false => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                false
                            }
                        },
                        false => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.parse_SLASH_F();
            self.parse_ExprSeq();
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "ExprSeq (SLASH_F ExprSeq)*"]
    pub(crate) fn test_ExprChoice(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::ExprChoice, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_ExprSeq() {
                true => true,
                false => false,
            }
        } {
            true => match {
                while {
                    let before_left = self.parser.mark();
                    match {
                        match self.test_SLASH_F() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => match {
                            match self.test_ExprSeq() {
                                true => true,
                                false => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                false
                            }
                        },
                        false => false,
                    }
                } {}
                true
            } {
                true => {
                    tracing::trace!(
                        "Recognized rule {} at {:?}",
                        "ExprChoice",
                        self.parser.mark()
                    );
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::ExprChoice, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::ExprChoice, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::ExprChoice, start, self.parser.mark());
        true
    }
    #[doc = "ExprPredicate+"]
    pub(crate) fn parse_ExprSeq(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::ExprSeq, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::ExprSeq);
        self.parse_ExprPredicate();
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    match self.test_ExprPredicate() {
                        true => true,
                        false => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.parse_ExprPredicate();
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "ExprPredicate+"]
    pub(crate) fn test_ExprSeq(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::ExprSeq, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let repeat_start = self.parser.mark();
        match true && {
            match self.test_ExprPredicate() {
                true => true,
                false => false,
            }
        } {
            true => {
                while {
                    match self.test_ExprPredicate() {
                        true => true,
                        false => false,
                    }
                } {}
                {
                    tracing::trace!("Recognized rule {} at {:?}", "ExprSeq", self.parser.mark());
                }
            }
            false => {
                self.parser.reset_to(repeat_start);
                {
                    self.parser.memoize_miss(MetaPegmeKind::ExprSeq, start);
                    self.parser.reset_to(start);
                    return false;
                }
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::ExprSeq, start, self.parser.mark());
        true
    }
    #[doc = "(AMPERSAND / EXCLAMATION)? ExprRepeat"]
    pub(crate) fn parse_ExprPredicate(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::ExprPredicate, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::ExprPredicate);
        if {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        match self.test_AMPERSAND() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            match self.test_EXCLAMATION() {
                                true => true,
                                false => false,
                            }
                        }
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            if {
                {
                    let before_lookahead = self.parser.mark();
                    match {
                        match self.test_AMPERSAND() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => {
                            self.parser.reset_to(before_lookahead);
                            true
                        }
                        false => false,
                    }
                }
            } {
                self.parse_AMPERSAND();
            } else {
                self.parse_EXCLAMATION();
            }
        }
        self.parse_ExprRepeat();
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "(AMPERSAND / EXCLAMATION)? ExprRepeat"]
    pub(crate) fn test_ExprPredicate(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::ExprPredicate, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                match self.test_AMPERSAND() {
                    true => true,
                    false => false,
                }
            } {
                true => true,
                false => {
                    self.parser.reset_to(before_left);
                    match self.test_EXCLAMATION() {
                        true => true,
                        false => true,
                    }
                }
            }
        } {
            true => match {
                match self.test_ExprRepeat() {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!(
                        "Recognized rule {} at {:?}",
                        "ExprPredicate",
                        self.parser.mark()
                    );
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser
                            .memoize_miss(MetaPegmeKind::ExprPredicate, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser
                    .memoize_miss(MetaPegmeKind::ExprPredicate, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::ExprPredicate, start, self.parser.mark());
        true
    }
    #[doc = "ExprAtom RepeatOp?"]
    pub(crate) fn parse_ExprRepeat(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::ExprRepeat, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::ExprRepeat);
        self.parse_ExprAtom();
        if {
            {
                let before_lookahead = self.parser.mark();
                match {
                    match self.test_RepeatOp() {
                        true => true,
                        false => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.parse_RepeatOp();
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "ExprAtom RepeatOp?"]
    pub(crate) fn test_ExprRepeat(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::ExprRepeat, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_ExprAtom() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.test_RepeatOp() {
                    true => true,
                    false => true,
                }
            } {
                true => {
                    tracing::trace!(
                        "Recognized rule {} at {:?}",
                        "ExprRepeat",
                        self.parser.mark()
                    );
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::ExprRepeat, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::ExprRepeat, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::ExprRepeat, start, self.parser.mark());
        true
    }
    #[doc = "(PAREN_L Expr PAREN_R / (((IDENT / DOT) / Keyword) / CharacterRanges))"]
    pub(crate) fn parse_ExprAtom(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::ExprAtom, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::ExprAtom);
        if {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        let before_left = self.parser.mark();
                        match {
                            match self.test_PAREN_L() {
                                true => true,
                                false => false,
                            }
                        } {
                            true => match {
                                match self.test_Expr() {
                                    true => true,
                                    false => false,
                                }
                            } {
                                true => true,
                                false => {
                                    self.parser.reset_to(before_left);
                                    false
                                }
                            },
                            false => false,
                        }
                    } {
                        true => match {
                            match self.test_PAREN_R() {
                                true => true,
                                false => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                false
                            }
                        },
                        false => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.parse_PAREN_L();
            self.parse_Expr();
            self.parse_PAREN_R();
        } else {
            if {
                {
                    let before_lookahead = self.parser.mark();
                    match {
                        let before_left = self.parser.mark();
                        match {
                            let before_left = self.parser.mark();
                            match {
                                match self.test_IDENT() {
                                    true => true,
                                    false => false,
                                }
                            } {
                                true => true,
                                false => {
                                    self.parser.reset_to(before_left);
                                    match self.test_DOT() {
                                        true => true,
                                        false => false,
                                    }
                                }
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                match self.test_Keyword() {
                                    true => true,
                                    false => false,
                                }
                            }
                        }
                    } {
                        true => {
                            self.parser.reset_to(before_lookahead);
                            true
                        }
                        false => false,
                    }
                }
            } {
                if {
                    {
                        let before_lookahead = self.parser.mark();
                        match {
                            let before_left = self.parser.mark();
                            match {
                                match self.test_IDENT() {
                                    true => true,
                                    false => false,
                                }
                            } {
                                true => true,
                                false => {
                                    self.parser.reset_to(before_left);
                                    match self.test_DOT() {
                                        true => true,
                                        false => false,
                                    }
                                }
                            }
                        } {
                            true => {
                                self.parser.reset_to(before_lookahead);
                                true
                            }
                            false => false,
                        }
                    }
                } {
                    if {
                        {
                            let before_lookahead = self.parser.mark();
                            match {
                                match self.test_IDENT() {
                                    true => true,
                                    false => false,
                                }
                            } {
                                true => {
                                    self.parser.reset_to(before_lookahead);
                                    true
                                }
                                false => false,
                            }
                        }
                    } {
                        self.parse_IDENT();
                    } else {
                        self.parse_DOT();
                    }
                } else {
                    self.parse_Keyword();
                }
            } else {
                self.parse_CharacterRanges();
            }
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "(PAREN_L Expr PAREN_R / (((IDENT / DOT) / Keyword) / CharacterRanges))"]
    pub(crate) fn test_ExprAtom(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::ExprAtom, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                let before_left = self.parser.mark();
                match {
                    match self.test_PAREN_L() {
                        true => true,
                        false => false,
                    }
                } {
                    true => match {
                        match self.test_Expr() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            false
                        }
                    },
                    false => false,
                }
            } {
                true => match {
                    match self.test_PAREN_R() {
                        true => true,
                        false => false,
                    }
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => {
                tracing::trace!("Recognized rule {} at {:?}", "ExprAtom", self.parser.mark());
            }
            false => {
                self.parser.reset_to(before_left);
                let before_left = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        let before_left = self.parser.mark();
                        match {
                            match self.test_IDENT() {
                                true => true,
                                false => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                match self.test_DOT() {
                                    true => true,
                                    false => false,
                                }
                            }
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            match self.test_Keyword() {
                                true => true,
                                false => false,
                            }
                        }
                    }
                } {
                    true => {
                        tracing::trace!(
                            "Recognized rule {} at {:?}",
                            "ExprAtom",
                            self.parser.mark()
                        );
                    }
                    false => {
                        self.parser.reset_to(before_left);
                        match self.test_CharacterRanges() {
                            true => {
                                tracing::trace!(
                                    "Recognized rule {} at {:?}",
                                    "ExprAtom",
                                    self.parser.mark()
                                );
                            }
                            false => {
                                self.parser.memoize_miss(MetaPegmeKind::ExprAtom, start);
                                self.parser.reset_to(start);
                                return false;
                            }
                        }
                    }
                }
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::ExprAtom, start, self.parser.mark());
        true
    }
    #[doc = "((STAR / PLUS) / QUESTION)"]
    pub(crate) fn parse_RepeatOp(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::RepeatOp, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::RepeatOp);
        if {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        match self.test_STAR() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            match self.test_PLUS() {
                                true => true,
                                false => false,
                            }
                        }
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            if {
                {
                    let before_lookahead = self.parser.mark();
                    match {
                        match self.test_STAR() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => {
                            self.parser.reset_to(before_lookahead);
                            true
                        }
                        false => false,
                    }
                }
            } {
                self.parse_STAR();
            } else {
                self.parse_PLUS();
            }
        } else {
            self.parse_QUESTION();
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "((STAR / PLUS) / QUESTION)"]
    pub(crate) fn test_RepeatOp(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::RepeatOp, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                match self.test_STAR() {
                    true => true,
                    false => false,
                }
            } {
                true => true,
                false => {
                    self.parser.reset_to(before_left);
                    match self.test_PLUS() {
                        true => true,
                        false => false,
                    }
                }
            }
        } {
            true => {
                tracing::trace!("Recognized rule {} at {:?}", "RepeatOp", self.parser.mark());
            }
            false => {
                self.parser.reset_to(before_left);
                match self.test_QUESTION() {
                    true => {
                        tracing::trace!(
                            "Recognized rule {} at {:?}",
                            "RepeatOp",
                            self.parser.mark()
                        );
                    }
                    false => {
                        self.parser.memoize_miss(MetaPegmeKind::RepeatOp, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::RepeatOp, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"\\\"\" (\"\\\" . / !\"\\\"\" .)* \"\\\"\""]
    pub(crate) fn parse_Keyword(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::Keyword, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::Keyword);
        self.parse_Trivia();
        self.parser.expect("\"");
        self.tree.push_tokens("\"");
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        let before_left = self.parser.mark();
                        match {
                            match self.parser.expect("\\") {
                                true => true,
                                false => false,
                            }
                        } {
                            true => match {
                                match self.parser.eat(|_| true) {
                                    Some(_) => true,
                                    None => false,
                                }
                            } {
                                true => true,
                                false => {
                                    self.parser.reset_to(before_left);
                                    false
                                }
                            },
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            let before_left = self.parser.mark();
                            match {
                                let before_predicate = self.parser.mark();
                                let predicate = {
                                    match self.parser.expect("\"") {
                                        true => true,
                                        false => false,
                                    }
                                };
                                self.parser.reset_to(before_predicate);
                                match predicate == false {
                                    true => true,
                                    false => false,
                                }
                            } {
                                true => match {
                                    match self.parser.eat(|_| true) {
                                        Some(_) => true,
                                        None => false,
                                    }
                                } {
                                    true => true,
                                    false => {
                                        self.parser.reset_to(before_left);
                                        false
                                    }
                                },
                                false => false,
                            }
                        }
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            if {
                {
                    let before_lookahead = self.parser.mark();
                    match {
                        let before_left = self.parser.mark();
                        match {
                            match self.parser.expect("\\") {
                                true => true,
                                false => false,
                            }
                        } {
                            true => match {
                                match self.parser.eat(|_| true) {
                                    Some(_) => true,
                                    None => false,
                                }
                            } {
                                true => true,
                                false => {
                                    self.parser.reset_to(before_left);
                                    false
                                }
                            },
                            false => false,
                        }
                    } {
                        true => {
                            self.parser.reset_to(before_lookahead);
                            true
                        }
                        false => false,
                    }
                }
            } {
                self.parser.expect("\\");
                self.tree.push_tokens("\\");
                self.tree.push_token(self.parser.eat(|_| true).unwrap());
            } else {
                self.tree.push_token(self.parser.eat(|_| true).unwrap());
            }
        }
        self.parser.expect("\"");
        self.tree.push_tokens("\"");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"\\\"\" (\"\\\" . / !\"\\\"\" .)* \"\\\"\""]
    pub(crate) fn test_Keyword(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::Keyword, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                let before_left = self.parser.mark();
                match {
                    match self.test_Trivia() {
                        true => true,
                        false => false,
                    }
                } {
                    true => match {
                        match self.parser.expect("\"") {
                            true => true,
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            false
                        }
                    },
                    false => false,
                }
            } {
                true => match {
                    while {
                        let before_left = self.parser.mark();
                        match {
                            let before_left = self.parser.mark();
                            match {
                                match self.parser.expect("\\") {
                                    true => true,
                                    false => false,
                                }
                            } {
                                true => match {
                                    match self.parser.eat(|_| true) {
                                        Some(_) => true,
                                        None => false,
                                    }
                                } {
                                    true => true,
                                    false => {
                                        self.parser.reset_to(before_left);
                                        false
                                    }
                                },
                                false => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                let before_left = self.parser.mark();
                                match {
                                    let before_predicate = self.parser.mark();
                                    let predicate = {
                                        match self.parser.expect("\"") {
                                            true => true,
                                            false => false,
                                        }
                                    };
                                    self.parser.reset_to(before_predicate);
                                    match predicate == false {
                                        true => true,
                                        false => false,
                                    }
                                } {
                                    true => match {
                                        match self.parser.eat(|_| true) {
                                            Some(_) => true,
                                            None => false,
                                        }
                                    } {
                                        true => true,
                                        false => {
                                            self.parser.reset_to(before_left);
                                            false
                                        }
                                    },
                                    false => false,
                                }
                            }
                        }
                    } {}
                    true
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("\"") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "Keyword", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::Keyword, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::Keyword, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::Keyword, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"[\" (CharacterRangesRange / CharacterRangesIdent)+ \"]\""]
    pub(crate) fn parse_CharacterRanges(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::CharacterRanges, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::CharacterRanges);
        self.parse_Trivia();
        self.parser.expect("[");
        self.tree.push_tokens("[");
        if {
            {
                let before_lookahead = self.parser.mark();
                match {
                    match self.test_CharacterRangesRange() {
                        true => true,
                        false => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.parse_CharacterRangesRange();
        } else {
            self.parse_CharacterRangesIdent();
        }
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        match self.test_CharacterRangesRange() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            match self.test_CharacterRangesIdent() {
                                true => true,
                                false => false,
                            }
                        }
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            if {
                {
                    let before_lookahead = self.parser.mark();
                    match {
                        match self.test_CharacterRangesRange() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => {
                            self.parser.reset_to(before_lookahead);
                            true
                        }
                        false => false,
                    }
                }
            } {
                self.parse_CharacterRangesRange();
            } else {
                self.parse_CharacterRangesIdent();
            }
        }
        self.parser.expect("]");
        self.tree.push_tokens("]");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"[\" (CharacterRangesRange / CharacterRangesIdent)+ \"]\""]
    pub(crate) fn test_CharacterRanges(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::CharacterRanges, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                let before_left = self.parser.mark();
                match {
                    match self.test_Trivia() {
                        true => true,
                        false => false,
                    }
                } {
                    true => match {
                        match self.parser.expect("[") {
                            true => true,
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            false
                        }
                    },
                    false => false,
                }
            } {
                true => match {
                    let repeat_start = self.parser.mark();
                    match true && {
                        let before_left = self.parser.mark();
                        match {
                            match self.test_CharacterRangesRange() {
                                true => true,
                                false => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                match self.test_CharacterRangesIdent() {
                                    true => true,
                                    false => false,
                                }
                            }
                        }
                    } {
                        true => {
                            while {
                                let before_left = self.parser.mark();
                                match {
                                    match self.test_CharacterRangesRange() {
                                        true => true,
                                        false => false,
                                    }
                                } {
                                    true => true,
                                    false => {
                                        self.parser.reset_to(before_left);
                                        match self.test_CharacterRangesIdent() {
                                            true => true,
                                            false => false,
                                        }
                                    }
                                }
                            } {}
                            true
                        }
                        false => {
                            self.parser.reset_to(repeat_start);
                            false
                        }
                    }
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("]") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!(
                        "Recognized rule {} at {:?}",
                        "CharacterRanges",
                        self.parser.mark()
                    );
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser
                            .memoize_miss(MetaPegmeKind::CharacterRanges, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser
                    .memoize_miss(MetaPegmeKind::CharacterRanges, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::CharacterRanges, start, self.parser.mark());
        true
    }
    #[doc = "(\"\\\" . / !\"]\" .)"]
    pub(crate) fn parse_CharacterRangesIdent(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::CharacterRangesIdent, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::CharacterRangesIdent);
        if {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        match self.parser.expect("\\") {
                            true => true,
                            false => false,
                        }
                    } {
                        true => match {
                            match self.parser.eat(|_| true) {
                                Some(_) => true,
                                None => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                false
                            }
                        },
                        false => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.parser.expect("\\");
            self.tree.push_tokens("\\");
            self.tree.push_token(self.parser.eat(|_| true).unwrap());
        } else {
            self.tree.push_token(self.parser.eat(|_| true).unwrap());
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "(\"\\\" . / !\"]\" .)"]
    pub(crate) fn test_CharacterRangesIdent(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::CharacterRangesIdent, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                match self.parser.expect("\\") {
                    true => true,
                    false => false,
                }
            } {
                true => match {
                    match self.parser.eat(|_| true) {
                        Some(_) => true,
                        None => false,
                    }
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => {
                tracing::trace!(
                    "Recognized rule {} at {:?}",
                    "CharacterRangesIdent",
                    self.parser.mark()
                );
            }
            false => {
                self.parser.reset_to(before_left);
                let before_left = self.parser.mark();
                match {
                    let before_predicate = self.parser.mark();
                    let predicate = {
                        match self.parser.expect("]") {
                            true => true,
                            false => false,
                        }
                    };
                    self.parser.reset_to(before_predicate);
                    match predicate == false {
                        true => true,
                        false => false,
                    }
                } {
                    true => match {
                        match self.parser.eat(|_| true) {
                            Some(_) => true,
                            None => false,
                        }
                    } {
                        true => {
                            tracing::trace!(
                                "Recognized rule {} at {:?}",
                                "CharacterRangesIdent",
                                self.parser.mark()
                            );
                        }
                        false => {
                            self.parser.reset_to(before_left);
                            {
                                self.parser
                                    .memoize_miss(MetaPegmeKind::CharacterRangesIdent, start);
                                self.parser.reset_to(start);
                                return false;
                            }
                        }
                    },
                    false => {
                        self.parser
                            .memoize_miss(MetaPegmeKind::CharacterRangesIdent, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            }
        };
        self.parser.memoize_match(
            MetaPegmeKind::CharacterRangesIdent,
            start,
            self.parser.mark(),
        );
        true
    }
    #[doc = "CharacterRangesIdent \"-\" CharacterRangesIdent"]
    pub(crate) fn parse_CharacterRangesRange(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::CharacterRangesRange, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::CharacterRangesRange);
        self.parse_CharacterRangesIdent();
        self.parser.expect("-");
        self.tree.push_tokens("-");
        self.parse_CharacterRangesIdent();
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "CharacterRangesIdent \"-\" CharacterRangesIdent"]
    pub(crate) fn test_CharacterRangesRange(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::CharacterRangesRange, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                match self.test_CharacterRangesIdent() {
                    true => true,
                    false => false,
                }
            } {
                true => match {
                    match self.parser.expect("-") {
                        true => true,
                        false => false,
                    }
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => match {
                match self.test_CharacterRangesIdent() {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!(
                        "Recognized rule {} at {:?}",
                        "CharacterRangesRange",
                        self.parser.mark()
                    );
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser
                            .memoize_miss(MetaPegmeKind::CharacterRangesRange, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser
                    .memoize_miss(MetaPegmeKind::CharacterRangesRange, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser.memoize_match(
            MetaPegmeKind::CharacterRangesRange,
            start,
            self.parser.mark(),
        );
        true
    }
    #[doc = "[ \\t\\n\\r]+"]
    pub(crate) fn parse_Whitespace(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::Whitespace, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::Whitespace);
        self.tree.push_token(self.parser.eat(|_| true).unwrap());
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    match self.parser.eat(|c| match c {
                        ' '..=' ' => true,
                        '\t'..='\t' => true,
                        '\n'..='\n' => true,
                        '\r'..='\r' => true,
                        _ => false,
                    }) {
                        Some(_) => true,
                        None => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.tree.push_token(self.parser.eat(|_| true).unwrap());
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "[ \\t\\n\\r]+"]
    pub(crate) fn test_Whitespace(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::Whitespace, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let repeat_start = self.parser.mark();
        match true && {
            match self.parser.eat(|c| match c {
                ' '..=' ' => true,
                '\t'..='\t' => true,
                '\n'..='\n' => true,
                '\r'..='\r' => true,
                _ => false,
            }) {
                Some(_) => true,
                None => false,
            }
        } {
            true => {
                while {
                    match self.parser.eat(|c| match c {
                        ' '..=' ' => true,
                        '\t'..='\t' => true,
                        '\n'..='\n' => true,
                        '\r'..='\r' => true,
                        _ => false,
                    }) {
                        Some(_) => true,
                        None => false,
                    }
                } {}
                {
                    tracing::trace!(
                        "Recognized rule {} at {:?}",
                        "Whitespace",
                        self.parser.mark()
                    );
                }
            }
            false => {
                self.parser.reset_to(repeat_start);
                {
                    self.parser.memoize_miss(MetaPegmeKind::Whitespace, start);
                    self.parser.reset_to(start);
                    return false;
                }
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::Whitespace, start, self.parser.mark());
        true
    }
    #[doc = "\"//\" (!EOL .)* EOL"]
    pub(crate) fn parse_Comment(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::Comment, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::Comment);
        self.parser.expect("//");
        self.tree.push_tokens("//");
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        let before_predicate = self.parser.mark();
                        let predicate = {
                            match self.test_EOL() {
                                true => true,
                                false => false,
                            }
                        };
                        self.parser.reset_to(before_predicate);
                        match predicate == false {
                            true => true,
                            false => false,
                        }
                    } {
                        true => match {
                            match self.parser.eat(|_| true) {
                                Some(_) => true,
                                None => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                false
                            }
                        },
                        false => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.tree.push_token(self.parser.eat(|_| true).unwrap());
        }
        self.parse_EOL();
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "\"//\" (!EOL .)* EOL"]
    pub(crate) fn test_Comment(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::Comment, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                match self.parser.expect("//") {
                    true => true,
                    false => false,
                }
            } {
                true => match {
                    while {
                        let before_left = self.parser.mark();
                        match {
                            let before_predicate = self.parser.mark();
                            let predicate = {
                                match self.test_EOL() {
                                    true => true,
                                    false => false,
                                }
                            };
                            self.parser.reset_to(before_predicate);
                            match predicate == false {
                                true => true,
                                false => false,
                            }
                        } {
                            true => match {
                                match self.parser.eat(|_| true) {
                                    Some(_) => true,
                                    None => false,
                                }
                            } {
                                true => true,
                                false => {
                                    self.parser.reset_to(before_left);
                                    false
                                }
                            },
                            false => false,
                        }
                    } {}
                    true
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => match {
                match self.test_EOL() {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "Comment", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::Comment, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::Comment, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::Comment, start, self.parser.mark());
        true
    }
    #[doc = "(Whitespace / Comment)*"]
    pub(crate) fn parse_Trivia(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::Trivia, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::Trivia);
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        match self.test_Whitespace() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            match self.test_Comment() {
                                true => true,
                                false => false,
                            }
                        }
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            if {
                {
                    let before_lookahead = self.parser.mark();
                    match {
                        match self.test_Whitespace() {
                            true => true,
                            false => false,
                        }
                    } {
                        true => {
                            self.parser.reset_to(before_lookahead);
                            true
                        }
                        false => false,
                    }
                }
            } {
                self.parse_Whitespace();
            } else {
                self.parse_Comment();
            }
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "(Whitespace / Comment)*"]
    pub(crate) fn test_Trivia(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::Trivia, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        while {
            let before_left = self.parser.mark();
            match {
                match self.test_Whitespace() {
                    true => true,
                    false => false,
                }
            } {
                true => true,
                false => {
                    self.parser.reset_to(before_left);
                    match self.test_Comment() {
                        true => true,
                        false => false,
                    }
                }
            }
        } {}
        {
            tracing::trace!("Recognized rule {} at {:?}", "Trivia", self.parser.mark());
        };
        self.parser
            .memoize_match(MetaPegmeKind::Trivia, start, self.parser.mark());
        true
    }
    #[doc = "(((\"\\n\" / \"\\r\\n\") / \"\\r\") / EOF)"]
    pub(crate) fn parse_EOL(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::EOL, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::EOL);
        if {
            {
                let before_lookahead = self.parser.mark();
                match {
                    let before_left = self.parser.mark();
                    match {
                        let before_left = self.parser.mark();
                        match {
                            match self.parser.expect("\n") {
                                true => true,
                                false => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                match self.parser.expect("\r\n") {
                                    true => true,
                                    false => false,
                                }
                            }
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            match self.parser.expect("\r") {
                                true => true,
                                false => false,
                            }
                        }
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            if {
                {
                    let before_lookahead = self.parser.mark();
                    match {
                        let before_left = self.parser.mark();
                        match {
                            match self.parser.expect("\n") {
                                true => true,
                                false => false,
                            }
                        } {
                            true => true,
                            false => {
                                self.parser.reset_to(before_left);
                                match self.parser.expect("\r\n") {
                                    true => true,
                                    false => false,
                                }
                            }
                        }
                    } {
                        true => {
                            self.parser.reset_to(before_lookahead);
                            true
                        }
                        false => false,
                    }
                }
            } {
                if {
                    {
                        let before_lookahead = self.parser.mark();
                        match {
                            match self.parser.expect("\n") {
                                true => true,
                                false => false,
                            }
                        } {
                            true => {
                                self.parser.reset_to(before_lookahead);
                                true
                            }
                            false => false,
                        }
                    }
                } {
                    self.parser.expect("\n");
                    self.tree.push_tokens("\n");
                } else {
                    self.parser.expect("\r\n");
                    self.tree.push_tokens("\r\n");
                }
            } else {
                self.parser.expect("\r");
                self.tree.push_tokens("\r");
            }
        } else {
            self.parse_EOF();
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "(((\"\\n\" / \"\\r\\n\") / \"\\r\") / EOF)"]
    pub(crate) fn test_EOL(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::EOL, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                let before_left = self.parser.mark();
                match {
                    match self.parser.expect("\n") {
                        true => true,
                        false => false,
                    }
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        match self.parser.expect("\r\n") {
                            true => true,
                            false => false,
                        }
                    }
                }
            } {
                true => true,
                false => {
                    self.parser.reset_to(before_left);
                    match self.parser.expect("\r") {
                        true => true,
                        false => false,
                    }
                }
            }
        } {
            true => {
                tracing::trace!("Recognized rule {} at {:?}", "EOL", self.parser.mark());
            }
            false => {
                self.parser.reset_to(before_left);
                match self.test_EOF() {
                    true => {
                        tracing::trace!("Recognized rule {} at {:?}", "EOL", self.parser.mark());
                    }
                    false => {
                        self.parser.memoize_miss(MetaPegmeKind::EOL, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::EOL, start, self.parser.mark());
        true
    }
    #[doc = "!."]
    pub(crate) fn parse_EOF(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::EOF, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::EOF);
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "!."]
    pub(crate) fn test_EOF(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::EOF, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_predicate = self.parser.mark();
        let predicate = {
            match self.parser.eat(|_| true) {
                Some(_) => true,
                None => false,
            }
        };
        self.parser.reset_to(before_predicate);
        match predicate == false {
            true => {
                tracing::trace!("Recognized rule {} at {:?}", "EOF", self.parser.mark());
            }
            false => {
                self.parser.memoize_miss(MetaPegmeKind::EOF, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::EOF, start, self.parser.mark());
        true
    }
    #[doc = "![a-zA-Z0-9_]"]
    pub(crate) fn parse_EOKW(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::EOKW, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::EOKW);
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "![a-zA-Z0-9_]"]
    pub(crate) fn test_EOKW(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::EOKW, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_predicate = self.parser.mark();
        let predicate = {
            match self.parser.eat(|c| match c {
                'a'..='z' => true,
                'A'..='Z' => true,
                '0'..='9' => true,
                '_'..='_' => true,
                _ => false,
            }) {
                Some(_) => true,
                None => false,
            }
        };
        self.parser.reset_to(before_predicate);
        match predicate == false {
            true => {
                tracing::trace!("Recognized rule {} at {:?}", "EOKW", self.parser.mark());
            }
            false => {
                self.parser.memoize_miss(MetaPegmeKind::EOKW, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::EOKW, start, self.parser.mark());
        true
    }
    #[doc = "Trivia !KW [a-zA-Z_] [a-zA-Z0-9_]*"]
    pub(crate) fn parse_IDENT(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::IDENT, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::IDENT);
        self.parse_Trivia();
        self.tree.push_token(self.parser.eat(|_| true).unwrap());
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    match self.parser.eat(|c| match c {
                        'a'..='z' => true,
                        'A'..='Z' => true,
                        '0'..='9' => true,
                        '_'..='_' => true,
                        _ => false,
                    }) {
                        Some(_) => true,
                        None => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.tree.push_token(self.parser.eat(|_| true).unwrap());
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia !KW [a-zA-Z_] [a-zA-Z0-9_]*"]
    pub(crate) fn test_IDENT(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::IDENT, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                let before_left = self.parser.mark();
                match {
                    match self.test_Trivia() {
                        true => true,
                        false => false,
                    }
                } {
                    true => match {
                        let before_predicate = self.parser.mark();
                        let predicate = {
                            match self.test_KW() {
                                true => true,
                                false => false,
                            }
                        };
                        self.parser.reset_to(before_predicate);
                        match predicate == false {
                            true => true,
                            false => false,
                        }
                    } {
                        true => true,
                        false => {
                            self.parser.reset_to(before_left);
                            false
                        }
                    },
                    false => false,
                }
            } {
                true => match {
                    match self.parser.eat(|c| match c {
                        'a'..='z' => true,
                        'A'..='Z' => true,
                        '_'..='_' => true,
                        _ => false,
                    }) {
                        Some(_) => true,
                        None => false,
                    }
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => match {
                while {
                    match self.parser.eat(|c| match c {
                        'a'..='z' => true,
                        'A'..='Z' => true,
                        '0'..='9' => true,
                        '_'..='_' => true,
                        _ => false,
                    }) {
                        Some(_) => true,
                        None => false,
                    }
                } {}
                true
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "IDENT", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::IDENT, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::IDENT, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::IDENT, start, self.parser.mark());
        true
    }
    #[doc = "RULE"]
    pub(crate) fn parse_KW(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::KW, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::KW);
        self.parse_RULE();
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "RULE"]
    pub(crate) fn test_KW(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::KW, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        match self.test_RULE() {
            true => {
                tracing::trace!("Recognized rule {} at {:?}", "KW", self.parser.mark());
            }
            false => {
                self.parser.memoize_miss(MetaPegmeKind::KW, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::KW, start, self.parser.mark());
        true
    }
    #[doc = "Trivia [0-9]+ ![a-zA-Z0-9_]"]
    pub(crate) fn parse_INTEGER(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::INTEGER, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::INTEGER);
        self.parse_Trivia();
        self.tree.push_token(self.parser.eat(|_| true).unwrap());
        while {
            {
                let before_lookahead = self.parser.mark();
                match {
                    match self.parser.eat(|c| match c {
                        '0'..='9' => true,
                        _ => false,
                    }) {
                        Some(_) => true,
                        None => false,
                    }
                } {
                    true => {
                        self.parser.reset_to(before_lookahead);
                        true
                    }
                    false => false,
                }
            }
        } {
            self.tree.push_token(self.parser.eat(|_| true).unwrap());
        }
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia [0-9]+ ![a-zA-Z0-9_]"]
    pub(crate) fn test_INTEGER(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::INTEGER, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            let before_left = self.parser.mark();
            match {
                match self.test_Trivia() {
                    true => true,
                    false => false,
                }
            } {
                true => match {
                    let repeat_start = self.parser.mark();
                    match true && {
                        match self.parser.eat(|c| match c {
                            '0'..='9' => true,
                            _ => false,
                        }) {
                            Some(_) => true,
                            None => false,
                        }
                    } {
                        true => {
                            while {
                                match self.parser.eat(|c| match c {
                                    '0'..='9' => true,
                                    _ => false,
                                }) {
                                    Some(_) => true,
                                    None => false,
                                }
                            } {}
                            true
                        }
                        false => {
                            self.parser.reset_to(repeat_start);
                            false
                        }
                    }
                } {
                    true => true,
                    false => {
                        self.parser.reset_to(before_left);
                        false
                    }
                },
                false => false,
            }
        } {
            true => match {
                let before_predicate = self.parser.mark();
                let predicate = {
                    match self.parser.eat(|c| match c {
                        'a'..='z' => true,
                        'A'..='Z' => true,
                        '0'..='9' => true,
                        '_'..='_' => true,
                        _ => false,
                    }) {
                        Some(_) => true,
                        None => false,
                    }
                };
                self.parser.reset_to(before_predicate);
                match predicate == false {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "INTEGER", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::INTEGER, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::INTEGER, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::INTEGER, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"rule\""]
    pub(crate) fn parse_RULE(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::RULE, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::RULE);
        self.parse_Trivia();
        self.parser.expect("rule");
        self.tree.push_tokens("rule");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"rule\""]
    pub(crate) fn test_RULE(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::RULE, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("rule") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "RULE", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::RULE, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::RULE, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::RULE, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"=\""]
    pub(crate) fn parse_EQUAL(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::EQUAL, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::EQUAL);
        self.parse_Trivia();
        self.parser.expect("=");
        self.tree.push_tokens("=");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"=\""]
    pub(crate) fn test_EQUAL(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::EQUAL, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("=") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "EQUAL", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::EQUAL, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::EQUAL, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::EQUAL, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"/\""]
    pub(crate) fn parse_SLASH_F(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::SLASH_F, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::SLASH_F);
        self.parse_Trivia();
        self.parser.expect("/");
        self.tree.push_tokens("/");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"/\""]
    pub(crate) fn test_SLASH_F(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::SLASH_F, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("/") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "SLASH_F", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::SLASH_F, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::SLASH_F, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::SLASH_F, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"&\""]
    pub(crate) fn parse_AMPERSAND(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::AMPERSAND, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::AMPERSAND);
        self.parse_Trivia();
        self.parser.expect("&");
        self.tree.push_tokens("&");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"&\""]
    pub(crate) fn test_AMPERSAND(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::AMPERSAND, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("&") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!(
                        "Recognized rule {} at {:?}",
                        "AMPERSAND",
                        self.parser.mark()
                    );
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::AMPERSAND, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::AMPERSAND, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::AMPERSAND, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"!\""]
    pub(crate) fn parse_EXCLAMATION(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::EXCLAMATION, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::EXCLAMATION);
        self.parse_Trivia();
        self.parser.expect("!");
        self.tree.push_tokens("!");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"!\""]
    pub(crate) fn test_EXCLAMATION(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::EXCLAMATION, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("!") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!(
                        "Recognized rule {} at {:?}",
                        "EXCLAMATION",
                        self.parser.mark()
                    );
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::EXCLAMATION, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::EXCLAMATION, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::EXCLAMATION, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"*\""]
    pub(crate) fn parse_STAR(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::STAR, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::STAR);
        self.parse_Trivia();
        self.parser.expect("*");
        self.tree.push_tokens("*");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"*\""]
    pub(crate) fn test_STAR(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::STAR, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("*") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "STAR", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::STAR, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::STAR, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::STAR, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"+\""]
    pub(crate) fn parse_PLUS(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::PLUS, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::PLUS);
        self.parse_Trivia();
        self.parser.expect("+");
        self.tree.push_tokens("+");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"+\""]
    pub(crate) fn test_PLUS(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::PLUS, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("+") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "PLUS", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::PLUS, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::PLUS, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::PLUS, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"?\""]
    pub(crate) fn parse_QUESTION(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::QUESTION, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::QUESTION);
        self.parse_Trivia();
        self.parser.expect("?");
        self.tree.push_tokens("?");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"?\""]
    pub(crate) fn test_QUESTION(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::QUESTION, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("?") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "QUESTION", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::QUESTION, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::QUESTION, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::QUESTION, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \".\""]
    pub(crate) fn parse_DOT(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::DOT, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::DOT);
        self.parse_Trivia();
        self.parser.expect(".");
        self.tree.push_tokens(".");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \".\""]
    pub(crate) fn test_DOT(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::DOT, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect(".") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "DOT", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::DOT, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::DOT, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::DOT, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \",\""]
    pub(crate) fn parse_COLON(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::COLON, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::COLON);
        self.parse_Trivia();
        self.parser.expect(",");
        self.tree.push_tokens(",");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \",\""]
    pub(crate) fn test_COLON(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::COLON, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect(",") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "COLON", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::COLON, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::COLON, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::COLON, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \";\""]
    pub(crate) fn parse_SEMICOLON(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::SEMICOLON, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::SEMICOLON);
        self.parse_Trivia();
        self.parser.expect(";");
        self.tree.push_tokens(";");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \";\""]
    pub(crate) fn test_SEMICOLON(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::SEMICOLON, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect(";") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!(
                        "Recognized rule {} at {:?}",
                        "SEMICOLON",
                        self.parser.mark()
                    );
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::SEMICOLON, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::SEMICOLON, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::SEMICOLON, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"(\""]
    pub(crate) fn parse_PAREN_L(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::PAREN_L, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::PAREN_L);
        self.parse_Trivia();
        self.parser.expect("(");
        self.tree.push_tokens("(");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"(\""]
    pub(crate) fn test_PAREN_L(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::PAREN_L, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("(") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "PAREN_L", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::PAREN_L, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::PAREN_L, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::PAREN_L, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \")\""]
    pub(crate) fn parse_PAREN_R(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::PAREN_R, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::PAREN_R);
        self.parse_Trivia();
        self.parser.expect(")");
        self.tree.push_tokens(")");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \")\""]
    pub(crate) fn test_PAREN_R(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::PAREN_R, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect(")") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "PAREN_R", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::PAREN_R, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::PAREN_R, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::PAREN_R, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"{\""]
    pub(crate) fn parse_BRACES_L(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::BRACES_L, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::BRACES_L);
        self.parse_Trivia();
        self.parser.expect("{");
        self.tree.push_tokens("{");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"{\""]
    pub(crate) fn test_BRACES_L(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::BRACES_L, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("{") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "BRACES_L", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::BRACES_L, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::BRACES_L, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::BRACES_L, start, self.parser.mark());
        true
    }
    #[doc = "Trivia \"}\""]
    pub(crate) fn parse_BRACES_R(&mut self) {
        let end = self
            .parser
            .memo(MetaPegmeKind::BRACES_R, self.parser.mark())
            .unwrap()
            .unwrap();
        let node = self.tree.start_node(MetaPegmeKind::BRACES_R);
        self.parse_Trivia();
        self.parser.expect("}");
        self.tree.push_tokens("}");
        self.parser.reset_to(end);
        self.tree.finish_node(node);
    }
    #[doc = "Trivia \"}\""]
    pub(crate) fn test_BRACES_R(&mut self) -> bool {
        let start = self.parser.mark();
        match self.parser.memo(MetaPegmeKind::BRACES_R, start) {
            Some(Some(end)) => {
                self.parser.reset_to(end);
                return true;
            }
            Some(None) => return false,
            None => {}
        }
        let before_left = self.parser.mark();
        match {
            match self.test_Trivia() {
                true => true,
                false => false,
            }
        } {
            true => match {
                match self.parser.expect("}") {
                    true => true,
                    false => false,
                }
            } {
                true => {
                    tracing::trace!("Recognized rule {} at {:?}", "BRACES_R", self.parser.mark());
                }
                false => {
                    self.parser.reset_to(before_left);
                    {
                        self.parser.memoize_miss(MetaPegmeKind::BRACES_R, start);
                        self.parser.reset_to(start);
                        return false;
                    }
                }
            },
            false => {
                self.parser.memoize_miss(MetaPegmeKind::BRACES_R, start);
                self.parser.reset_to(start);
                return false;
            }
        };
        self.parser
            .memoize_match(MetaPegmeKind::BRACES_R, start, self.parser.mark());
        true
    }
}
