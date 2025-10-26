use pegme_core::grammar::Grammar;

pub fn make_meta_grammar() -> Grammar {
    use pegme_core::grammar::dsl::*;

    let range_ident_start = _ranges(vec!['a'..='z', 'A'..='Z', '_'..='_']);
    let range_ident_continue = _ranges(vec!['a'..='z', 'A'..='Z', '0'..='9', '_'..='_']);

    grammar! {
        let file = item_rule.cast().star() - trivia - EOF;
        let item_rule = rule_kind - IDENT - EQUAL - expr - SEMICOLON;
        let rule_kind = RULE | TOKEN;

        // Expressions.
        let expr = expr_choice;
        let expr_choice = expr_seq - _star(SLASH_F - expr_seq);
        let expr_seq = _plus(expr_predicate);
        let expr_predicate = _opt(predicate_op) - expr_repeat;
        let expr_repeat = expr_atom - _opt(repeat_op);
        let expr_atom = (PAREN_L - expr - PAREN_R) | IDENT | DOT | LITERAL | ranges;

        let predicate_op = AND | BANG;
        let repeat_op = STAR | PLUS | QUESTION;

        let ranges = trivia - "[" - _star(range_range | RANGE_SOLO) - "]";
        let range_range = RANGE_SOLO - "-" - RANGE_SOLO;
        #[token]
        let RANGE_SOLO = "\\" - _any() | _not("]") - _any();

        // Whitespace.
        let trivia = _star(WHITESPACE | COMMENT);
        #[token]
        let WHITESPACE = _plus(_ranges(vec![' '..=' ', '\t'..='\t', '\r'..='\r', '\n'..='\n']));
        #[token]
        let COMMENT = "//" - _star(_not(EOL) - _any()) - EOL;
        #[token]
        let EOL = _eps() - "\n" | "\r\n" | "\r" | EOF;
        #[token]
        let EOF = _not(_any());
        let eokw = _not(range_ident_continue.clone());

        // Variadic tokens.
        #[token]
        let LITERAL = trivia - "\"" - _star(
            "\\" - _any() // escape sequence
            | _not("\"") - _any()
        ) - "\"";
        #[token]
        let IDENT = trivia - _not(kw) - range_ident_start.clone() - _star(range_ident_continue.clone());
        let kw = RULE;

        // Static tokens.
        #[token]
        let RULE = trivia - "rule";
        #[token]
        let TOKEN = trivia - "token";
        #[token]
        let EQUAL = trivia - "=";
        #[token]
        let SLASH_F = trivia - "/";
        #[token]
        let AND = trivia - "&";
        #[token]
        let BANG = trivia - "!";
        #[token]
        let STAR = trivia - "*";
        #[token]
        let PLUS = trivia - "+";
        #[token]
        let QUESTION = trivia - "?";
        #[token]
        let DOT = trivia - ".";
        #[token]
        let SEMICOLON = trivia - ";";
        #[token]
        let PAREN_L = trivia - "(";
        #[token]
        let PAREN_R = trivia - ")";
    }.unwrap()
}
