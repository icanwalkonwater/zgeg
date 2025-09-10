use pegger::PegGrammar;

#[allow(non_snake_case)]
pub fn make_zgeg_grammar() -> PegGrammar {
    use pegger::dsl::*;

    let mut g = PegGrammarBuilder::default();
    setup_rules!(g;
        File, Item,
        ItemFunction,
        Block, Statement,
        Expr, ExprAtom,
        ExprInfixOp, ExprPrefixOp, ExprPostfixOp,

        // Tokens

        Ident,
        Number, INTEGER, FLOATING,
        String,
        Comment, Trivia,
        EOF, EOL,
        SEMICOLON,
        PLUS, PLUSPLUS, HYPHEN, HYPHENHYPHEN, STAR, STARSTAR, SLASH_F,
        QUOTE_S, QUOTE_D,
        PAREN_L, PAREN_R,
        BRACES_L, BRACES_R,
        EOKW, KW,
        KW_fun,
    );

    File += &Trivia + star(&Item) + &EOF;

    Item += &ItemFunction;

    ItemFunction += &KW_fun + &Ident + &PAREN_L + &PAREN_R + &Block;

    Block += &BRACES_L + star(&Statement) + &BRACES_R;

    Statement += &Expr + &SEMICOLON;

    Expr += opt(&ExprPrefixOp) + &ExprAtom + opt(&ExprPostfixOp) + opt(&ExprInfixOp + &Expr);
    ExprAtom += &PAREN_L + &Expr + &PAREN_R;
    ExprAtom += &Number;
    ExprAtom += &String;

    ExprInfixOp += &PLUS;
    ExprInfixOp += &HYPHEN;
    ExprInfixOp += &STAR;
    ExprInfixOp += &SLASH_F;
    ExprPrefixOp += &PLUS;
    ExprPrefixOp += &HYPHEN;

    // Tokens.
    // They also eat up any trivia right after them.

    Ident += not(&KW) + (XidStart | "_") + star(XidContinue) + &Trivia;

    Number += &INTEGER;
    Number += &FLOATING;

    INTEGER += plus(['0', '9']) + not(Alphanumeric) + &Trivia;
    FLOATING += plus(['0', '9'])
        + "."
        + star(['0', '9'])
        + (eps() + "f" | "F")
        + not(Alphanumeric)
        + &Trivia;

    String += &QUOTE_S + star(not(&QUOTE_S) + not(&EOL) + any()) + &QUOTE_S;
    String += &QUOTE_D + star(not(&QUOTE_D) + not(&EOL) + any()) + &QUOTE_D;

    Comment += "//" + star(not(&EOL) + any()) + &EOL;
    Trivia += star(Whitespace | &Comment);

    EOF += not(any());
    EOL += "\n";
    EOL += "\r\n";
    EOL += "\r";
    EOL += &EOF;

    SEMICOLON += ";" + &Trivia;
    PLUS += "+" + not("+") + &Trivia;
    PLUSPLUS += "++" + &Trivia;
    HYPHEN += "-" + not("-") + &Trivia;
    HYPHENHYPHEN += "--" + &Trivia;
    STAR += "*" + not("*") + &Trivia;
    STARSTAR += "**" + &Trivia;
    QUOTE_S += "'" + &Trivia;
    QUOTE_D += "\"" + &Trivia;
    SLASH_F += "/" + not("/") + &Trivia;
    PAREN_L += "(" + &Trivia;
    PAREN_R += ")" + &Trivia;
    BRACES_L += "{" + &Trivia;
    BRACES_R += "}" + &Trivia;

    // End-of-keyword, helper to avoid issues like "funhello" being matched by `"fun" ident`
    EOKW += not(Alphanumeric | "_") + &Trivia;
    KW += &KW_fun;
    KW_fun += "fun" + &EOKW;

    g.build()
}
