use pegger::PegGrammar;

pub fn make_zgeg_grammar() -> PegGrammar {
    use pegger::dsl::*;

    let mut g = PegGrammarBuilder::default();
    g.set_trivia_rule_name("trivia");
    setup_rules!(g;
        source_file,
        item, item_function,

        argument_list,

        block, instruction,
        expression,
        function_call,

        ty,

        ident, ident_prefix, ident_body,
        whitespace, eol,
        trivia,
    );

    source_file += star(EPS + &item) + eof();
    item += &item_function;

    item_function += EPS - "fun" - &trivia - &ident + "(" + ")" + &block;

    block += EPS - "{" - star(EPS + &instruction) + "}";
    instruction += EPS - &expression + ";";

    expression += EPS - &function_call;

    function_call += EPS - &ident + "(" + ")";

    ty += &ident;

    ident += EPS - &ident_prefix - star(&ident_body);
    // Ident don't start with numbers.
    ident_prefix += not('0'..='9') - &ident_body;
    ident_body += "_";
    ident_body += 'a'..='z';
    ident_body += 'A'..='Z';
    ident_body += '0'..='9';

    whitespace += " ";
    whitespace += '\x09'..='\x0d';
    eol += "\r\n";
    eol += "\n";
    eol += "\r";

    trivia += (EPS - &whitespace).plus() - opt(&trivia);
    // EOL is optional because it can be the end of file.
    trivia += "//" - (not(&eol) - ANY).star() - opt(&eol) - opt(&trivia);
    trivia += "/*" - (not("*/") - ANY).star() - "*/" - opt(&trivia);

    g.build()
}
