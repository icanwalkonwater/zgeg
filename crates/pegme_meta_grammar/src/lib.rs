use pegme_core::grammar::{dsl::*, PegGrammar};

#[allow(non_snake_case)]
pub fn make_meta_grammar() -> PegGrammar {
    let mut g = PegGrammarBuilder::default();
    declare_rules! {
        g;
        File,
        Rule, RuleKind,

        Expr, ExprChoice, ExprSeq, ExprPredicate, ExprRepeat, ExprAtom,
        RepeatOp,
        CharacterRanges,
        CharacterRangesIdent,
        CharacterRangesRange,

        Whitespace, Comment, Trivia,
        EOL, EOF, EOKW,

        LITERAL_KEYWORD, IDENT, KW, INTEGER,
        RULE,
        EQUAL, SLASH_F, AMPERSAND, EXCLAMATION, STAR, PLUS, QUESTION, DOT,
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
    ExprRepeat += &ExprAtom + opt(&RepeatOp);
    ExprAtom += &PAREN_L + &Expr + &PAREN_R;
    ExprAtom += &IDENT | &DOT | &LITERAL_KEYWORD | &CharacterRanges;

    RepeatOp += &STAR | &PLUS | &QUESTION;

    CharacterRanges += &Trivia + "[" + plus(&CharacterRangesRange | &CharacterRangesIdent) + "]";
    CharacterRangesIdent += "\\" + any();
    CharacterRangesIdent += not("]") + any();
    CharacterRangesRange += &CharacterRangesIdent + "-" + &CharacterRangesIdent;

    Whitespace += plus(class(" \t\n\r"));
    Comment += "//" + star(not(&EOL) + any()) + &EOL;
    Trivia += star(&Whitespace | &Comment);
    EOL += eps() + "\n" | "\r\n" | "\r" | &EOF;
    EOF += not(any());
    EOKW += not(class("a-zA-Z0-9_"));

    LITERAL_KEYWORD += &Trivia + "\"" + star("\\" + any() | not("\"") + any()) + "\"";
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
    QUESTION += &Trivia + "?";
    DOT += &Trivia + ".";
    COLON += &Trivia + ",";
    SEMICOLON += &Trivia + ";";
    PAREN_L += &Trivia + "(";
    PAREN_R += &Trivia + ")";
    BRACES_L += &Trivia + "{";
    BRACES_R += &Trivia + "}";

    g.build()
}
