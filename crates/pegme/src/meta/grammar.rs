use crate::grammar::PegGrammar;

#[allow(non_snake_case)]
pub fn make_meta_grammar() -> PegGrammar {
    use crate::grammar::dsl::*;
    let mut g = PegGrammarBuilder::default();
    declare_rules! {
        g;
        File,
        Rule, RuleKind,

        Expr, ExprChoice, ExprSeq, ExprPredicate, ExprRepeat, ExprAtom,
        RepeatOp,
        Keyword, CharacterRanges,

        Whitespace, Comment, Trivia,
        EOL, EOF, EOKW,

        IDENT, KW, INTEGER,
        RULE,
        EQUAL, SLASH_F, AMPERSAND, EXCLAMATION, STAR, PLUS, DOT,
        COLON, SEMICOLON,
        PAREN_L, PAREN_R,
        BRACES_L, BRACES_R,
    };

    File += star(&Rule) + &Trivia + &EOF;
    Rule += &RuleKind + &IDENT + &EQUAL + &Expr + &SEMICOLON;
    RuleKind += &RULE;

    Expr += &ExprChoice;
    ExprChoice += &ExprSeq + star(&SLASH_F + &ExprSeq);
    ExprSeq += plus(&ExprPredicate);
    ExprPredicate += opt(&AMPERSAND | &EXCLAMATION) + &ExprRepeat;
    ExprRepeat += &ExprAtom + &RepeatOp;
    ExprAtom += &PAREN_L + &Expr + &PAREN_R;
    ExprAtom += &IDENT | &DOT | &Keyword | &CharacterRanges;

    RepeatOp += &STAR | &PLUS;
    Keyword += &Trivia + "\"" + star("\\" + any() | not("\"" + any())) + "\"";
    CharacterRanges +=
        &Trivia + "[" + star(any() + "-" + not("!") + any() | not("]" + any())) + "]";

    Whitespace += plus(class(" \t\n\r"));
    Comment += "//" + star(not(&EOL) + any()) + &EOL;
    Trivia += star(&Whitespace | &Comment);
    EOL += eps() + "\n" | "\r\n" | "\r" | &EOF;
    EOF += not(any());
    EOKW += not(class("a-zA-Z0-9_"));

    IDENT += &Trivia + not(&KW) + class("a-zA-Z_") + star(class("a-zA-Z0-9_"));
    KW += &RULE;
    INTEGER += &Trivia + plus(class("0-9")) + not(class("a-zA-Z0-9_"));

    RULE += &Trivia + "rule";
    EQUAL += &Trivia + "=";
    SLASH_F += &Trivia + "/";
    AMPERSAND += &Trivia + "&";
    EXCLAMATION += &Trivia + "!";
    STAR += &Trivia + "*";
    PLUS += &Trivia + "+";
    DOT += &Trivia + ".";
    COLON += &Trivia + ",";
    SEMICOLON += &Trivia + ";";
    PAREN_L += &Trivia + "(";
    PAREN_R += &Trivia + ")";
    BRACES_L += &Trivia + "{";
    BRACES_R += &Trivia + "}";

    g.build()
}
