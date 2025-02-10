/**
 * @file Zgeg's official (TM) parser
 * @author icanwalkonwater
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "zgeg",

  precedences: _ => [
    ["unary", "factor", "sum", "bin", "bool"],
  ],

  word: $ => $.identifier,

  rules: {
    source_file: $ => repeat($._item),

    _item: $ => choice(
      $.function_item,
    ),

    function_item: $ => seq(
      "fun",
      field("name", $.identifier),
      "(", ")",
      optional(seq("->", field("return_type", $.type))),
      field("body", $.block),
    ),

    block: $ => seq(
      "{",
      field("statements", repeat($._statement)),
      "}",
    ),

    _statement: $ => choice(
      $._item,
      $.let_statement,
      $.assignement_statement,
      $.return_statement,
      $._expression_statement,
    ),

    _expression_statement: $ => choice(
      seq($._block_expression, ";"),
      seq($._no_block_expression, ";"),
    ),

    let_statement: $ => seq(
      "let",
      field("name", $.identifier),
      optional(seq(
        ":",
        field("type", $.type),
      )),
      optional(seq(
        "=",
        field("initializer", $._expression),
      )),
      ";"
    ),

    assignement_statement: $ => seq(
      field("place", $.identifier),
      "=",
      field("value", $._expression),
      ";",
    ),

    return_statement: $ => seq(
      "return",
      optional(field("return_value", $._expression)),
      ";",
    ),

    _expression: $ => choice($._no_block_expression, $._block_expression),

    _no_block_expression: $ => choice(
      $.binary_expression,
      $.unary_expression,
      $.function_call,
      $.identifier,
      $._literal,
      seq("(", $._expression, ")"),
    ),

    _block_expression: $ => choice(
      $.if_expression,
      $.loop_expression,
    ),

    if_expression: $ => seq(
      "if",
      $._expression,
      $.block,
      repeat(seq(
        "else", "if",
        $._expression,
        $.block,
      )),
      optional(seq(
        "else",
        $.block,
      )),
    ),

    loop_expression: $ => seq(
      "loop",
      $.block,
    ),

    unary_expression: $ => prec("unary", seq(
      choice("-", "+", "!", "~"),
      $._expression,
    )),

    binary_expression: $ => choice(
      prec.left("factor", seq(field("left", $._expression), "*", field("right", $._expression))),
      prec.left("factor", seq(field("left", $._expression), "/", field("right", $._expression))),
      prec.left("sum", seq(field("left", $._expression), "+", field("right", $._expression))),
      prec.left("sum", seq(field("left", $._expression), "-", field("right", $._expression))),
      prec.left("bool", seq(field("left", $._expression), "==", field("right", $._expression))),
      prec.left("bool", seq(field("left", $._expression), "!=", field("right", $._expression))),
      prec.left("bool", seq(field("left", $._expression), "<", field("right", $._expression))),
      prec.left("bool", seq(field("left", $._expression), ">", field("right", $._expression))),
      prec.left("bool", seq(field("left", $._expression), "<=", field("right", $._expression))),
      prec.left("bool", seq(field("left", $._expression), ">=", field("right", $._expression))),
      prec.left("bool", seq(field("left", $._expression), "&&", field("right", $._expression))),
      prec.left("bool", seq(field("left", $._expression), "||", field("right", $._expression))),
    ),

    function_call: $ => seq(
      field("name", $.identifier),
      "(",
      field("arguments", optional(seq(
        $._expression,
        repeat(seq(",", $._expression)),
        optional(","),
      ))),
      ")",
    ),

    type: $ => choice($.identifier, $.builtin_type),

    builtin_type: _ => choice(
      "u8", "i8",
      "u16", "i16",
      "u32", "i32",
      "u64", "i64",
    ),

    identifier: _ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    _literal: $ => choice(
      $.number_literal,
      $.string_literal,
    ),

    number_literal: $ => /\d+/,
    string_literal: $ => /"[^"]*"/,
  }
});
