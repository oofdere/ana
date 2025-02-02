/**
 * @file atproto lexicon dsl
 * @author Tibet Tornaci <oofdere@hey.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "ana",

  word: $ => $.id,

  rules: {
    program: $ => repeat(choice($.scope)),

    array: $ => seq("[", optional($.slice), "]"),
    
    nsid: $ => seq($.id, repeat1(seq(".", $.id))),

    integer: $ => /\d+/,

    slice: $ => seq(
      optional(field("min", $.integer)),
      "..",
      optional(field("max", $.integer))
    ),

    block: $ => seq("{", 
      choice(repeat($.param), 
      repeat(choice($.scope, $.record))), 
      "}"
    ),
    scope: $ => seq(
      choice($.id, $.nsid), 
      $.block
    ),
    record: $ => seq("record", $.scope),
    
    params: $ => seq("(", repeat($.param), ")"),

    param: $ => seq(
      choice(
        seq(
          field("ref", $.ref),
          optional(field("optional", "?")),
        ),
        seq(
          field("name", $.id),
          optional(field("optional", "?")),
          choice(":", "="), 
          choice($.ref, $.type, $.slice, $.string), 
        )
      ),
      optional(field("array", $.array)),
      optional(choice(",", ";")),
    ),

    type: $ => seq(
      $.id, 
      optional($.params)
    ),

    ref: $ => seq(
      "#", 
      field("id", choice($.id, $.nsid))
    ),

    string: $ => seq(
      '"', 
      field("contents", /(?:[^"\\]|\\.)*/), 
      '"'
    ),

    id: $ => /[a-zA-Z_$][a-zA-Z0-9_$]*/
  }
});
