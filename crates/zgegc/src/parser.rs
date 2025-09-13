use pegger::grammar::PegGrammar;

#[allow(non_snake_case)]
pub fn make_zgeg_grammar() -> PegGrammar {
    use pegger::grammar::dsl::*;

    let mut g = PegGrammarBuilder::default();
    declare_rules!(g;
        File, Item,
        ItemFunction,
        Block, Statement,
        Expr, ExprAtom,
        ExprInfixOp, ExprPrefixOp, ExprPostfixOp,
        FunctionCall,

        // Tokens

        Ident,
        Number, INTEGER, FLOATING,
        STRING,
        Comment, Trivia,
        EOF, EOL,
        DOT, DOTDOT, COMMA, SEMICOLON,
        PLUS, PLUSPLUS, HYPHEN, HYPHENHYPHEN, STAR, STARSTAR, SLASH_F,
        QUOTE_S, QUOTE_D,
        PAREN_L, PAREN_R,
        BRACES_L, BRACES_R,
        EOKW, KW,
        KW_fun,
    );

    File += star(&Item) + &Trivia + &EOF;

    Item += &ItemFunction;

    ItemFunction += &KW_fun + &Ident + &PAREN_L + &PAREN_R + &Block;

    Block += &BRACES_L + star(&Statement) + &BRACES_R;

    Statement += &Expr + &SEMICOLON;

    // Written to be post processed by a Pratt parser.
    Expr += opt(&ExprPrefixOp) + &ExprAtom + opt(&ExprPostfixOp) + opt(&ExprInfixOp + &Expr);
    ExprAtom += &PAREN_L + &Expr + &PAREN_R;
    ExprAtom += &Number;
    ExprAtom += &STRING;
    ExprAtom += &FunctionCall;

    ExprInfixOp += &PLUS;
    ExprInfixOp += &HYPHEN;
    ExprInfixOp += &STAR;
    ExprInfixOp += &SLASH_F;
    ExprPrefixOp += &PLUS;
    ExprPrefixOp += &HYPHEN;

    FunctionCall += &Ident + &PAREN_L + &PAREN_R;
    FunctionCall += &Ident + &PAREN_L + &Expr + star(&COMMA + &Expr) + opt(&COMMA) + &PAREN_R;

    // Tokens.
    // They also eat up any trivia right after them.

    Ident += &Trivia + not(&KW) + (Utf8XidStart | "_") + star(Utf8XidContinue);

    Number += &INTEGER;
    Number += &FLOATING;

    let c09 = || class("0-9");
    let cazAZ09 = || class("a-zA-Z0-9");

    INTEGER += &Trivia + plus(c09()) + not(cazAZ09());
    FLOATING += &Trivia + plus(c09()) + "." + star(c09()) + class("fF") + not(cazAZ09());

    STRING += &Trivia + "'" + star(not(&QUOTE_S) + any()) + "'";
    STRING += &Trivia + "\"" + star(not(&QUOTE_D) + any()) + "\"";

    Comment += "//" + star(not(&EOL) + any()) + &EOL;
    Trivia += star(Utf8Whitespace | &Comment);

    EOF += not(any());
    EOL += "\n";
    EOL += "\r\n";
    EOL += "\r";

    DOT += &Trivia + "." + not(".");
    DOTDOT += &Trivia + "..";
    COMMA += &Trivia + ",";
    SEMICOLON += &Trivia + ";";
    PLUS += &Trivia + "+" + not("+");
    PLUSPLUS += &Trivia + "++";
    HYPHEN += &Trivia + "-" + not("-");
    HYPHENHYPHEN += &Trivia + "--";
    STAR += &Trivia + "*" + not("*");
    STARSTAR += &Trivia + "**";
    QUOTE_S += &Trivia + "'";
    QUOTE_D += &Trivia + "\"";
    SLASH_F += &Trivia + "/" + not("/");
    PAREN_L += &Trivia + "(";
    PAREN_R += &Trivia + ")";
    BRACES_L += &Trivia + "{";
    BRACES_R += &Trivia + "}";

    // End-of-keyword, helper to avoid issues like "funhello" being matched by `"fun" ident`
    EOKW += not(class("a-zA-Z0-9_"));
    KW += &KW_fun;
    KW_fun += &Trivia + "fun" + &EOKW;

    g.build()
}
