/**
 * @file Zgeg's official (TM) parser
 * @author icanwalkonwater
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "zgeg",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => "hello"
  }
});
