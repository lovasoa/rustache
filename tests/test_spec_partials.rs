extern crate rustache;

use rustache::{HashBuilder, Render, VecBuilder};
use std::io::Cursor;

//   - name: Basic Behavior
//     desc: The greater-than operator should expand to the named partial.
//     data: { }
//     template: '"{{>text}}"'
//     partials: { text: 'from partial' }
//     expected: '"from partial"'
#[test]
fn test_spec_partials_basic_behavior() {
    let data = HashBuilder::new();
    let mut rv = Cursor::new(Vec::new());
    data.render("\"{{>test_data/test_spec_partials_basic}}\"", &mut rv).unwrap();

    assert_eq!("\"from partial\"".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Failed Lookup
//     desc: The empty string should be used when the named partial is not found.
//     data: { }
//     template: '"{{>text}}"'
//     partials: { }
//     expected: '""'
#[test]
fn test_spec_partials_failed_lookup() {
    let data = HashBuilder::new();
    let mut rv = Cursor::new(Vec::new());
    data.render("\"{{>text}}\"", &mut rv).unwrap();

    assert_eq!("\"\"".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Context
//     desc: The greater-than operator should operate within the current context.
//     data: { text: 'content' }
//     template: '"{{>partial}}"'
//     partials: { partial: '*{{text}}*' }
//     expected: '"*content*"'
#[test]
fn test_spec_partials_context() {
    let data = HashBuilder::new().insert("text", "content");
    let mut rv = Cursor::new(Vec::new());
    data.render("\"{{>test_data/test_spec_partials_context}}\"", &mut rv).unwrap();

    assert_eq!("\"*content*\"".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Recursion
//     desc: The greater-than operator should properly recurse.
//     data: { content: "X", nodes: [ { content: "Y", nodes: [] } ] }
//     template: '{{>node}}'
//     partials: { node: '{{content}}<{{#nodes}}{{>node}}{{/nodes}}>' }
//     expected: 'X<Y<>>'
#[test]
fn test_spec_partials_recursion() {
    let data = HashBuilder::new()
        .insert("content", "X")
        .insert("nodes", VecBuilder::new()
            .push(HashBuilder::new()
                .insert("content", "Y")
                    .insert("nodes", VecBuilder::new()
                    )
            )
        );
    let mut rv = Cursor::new(Vec::new());

    data.render("{{>test_data/test_spec_partials_recursion}}", &mut rv).unwrap();

    assert_eq!("X<Y<>>".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Surrounding Whitespace
//     desc: The greater-than operator should not alter surrounding whitespace.
//     data: { }
//     template: '| {{>partial}} |'
//     partials: { partial: "\t|\t" }
//     expected: "| \t|\t |"
#[test]
fn test_spec_partials_surrounding_whitespace() {
    let data = HashBuilder::new();
    let mut rv = Cursor::new(Vec::new());
    data.render("| {{>test_data/test_spec_partials_whitespace}} |", &mut rv).unwrap();

    assert_eq!("| \t|\t |".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Inline Indentation
//     desc: Whitespace should be left untouched.
//     data: { data: '|' }
//     template: "  {{data}}  {{> partial}}\n"
//     partials: { partial: ">\n>" }
//     expected: "  |  >\n>\n"
#[test]
fn test_spec_partials_inline_indentation() {
    let data = HashBuilder::new().insert("data", "|");
    let mut rv = Cursor::new(Vec::new());
    data.render("  {{data}}  {{> test_data/test_spec_partials_inline_indentation}}\n", &mut rv).unwrap();

    assert_eq!("  |  >\n>\n".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Standalone Line Endings
//     desc: '"\r\n" should be considered a newline for standalone tags.'
//     data: { }
//     template: "|\r\n{{>partial}}\r\n|"
//     partials: { partial: ">" }
//     expected: "|\r\n>|"
#[test]
#[ignore]
fn test_spec_partials_standalone_line_endings() {
    let data = HashBuilder::new();
    let mut rv = Cursor::new(Vec::new());
    data.render("|\r\n{{>partial}}\r\n|", &mut rv).unwrap();

    assert_eq!("|\r\n>|".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Standalone Without Previous Line
//     desc: Standalone tags should not require a newline to precede them.
//     data: { }
//     template: "  {{>partial}}\n>"
//     partials: { partial: ">\n>"}
//     expected: "  >\n  >>" --> These results appear to be erroneous
#[test]
fn test_spec_partials_standalone_without_previous_line() {
    let data = HashBuilder::new();
    let mut rv = Cursor::new(Vec::new());
    data.render("  {{>test_data/test_spec_partials_standalone_without_previous_line}}\n>", &mut rv).unwrap();

    assert_eq!("  >\n>\n>".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Standalone Without Newline
//     desc: Standalone tags should not require a newline to follow them.
//     data: { }
//     template: ">\n  {{>partial}}"
//     partials: { partial: ">\n>" }
//     expected: ">\n  >\n  >" --> is this test really right????
#[test]
fn test_spec_partials_standalone_without_newline() {
    let data = HashBuilder::new();
    let mut rv = Cursor::new(Vec::new());
    data.render(">\n  {{>test_data/test_spec_partials_standalone_without_newline}}", &mut rv).unwrap();

    assert_eq!(">\n  >\n>".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Standalone Indentation
//     desc: Each line of the partial should be indented before rendering.
//     data: { content: "<\n->" }
//     template: |
//       \
//        {{>partial}}
//       /
//     partials:
//       partial: |
//         |
//         {{{content}}}
//         |
//     expected: |
//       \
//        |
//        <
//       ->
//        |
//       /
#[test]
#[ignore]
fn test_spec_partials_standalone_indentation() {
    let data = HashBuilder::new().insert("content", "<\n->");

    let mut rv = Cursor::new(Vec::new());
    data.render("|\n\\\n {{>test_data/test_spec_partials_standalone_indentation}}\n/\n", &mut rv).unwrap();

    assert_eq!("|\n\\\n |\n <\n ->\n |\n/\n".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}

//   - name: Padding Whitespace
//     desc: Superfluous in-tag whitespace should be ignored.
//     data: { boolean: true }
//     template: "|{{> partial }}|"
//     partials: { partial: "[]" }
//     expected: '|[]|'
#[test]
fn test_spec_partials_padding_whitespace() {
    let data = HashBuilder::new()
                .insert("boolean", true);
    let mut rv = Cursor::new(Vec::new());
    data.render("|{{> test_data/test_spec_partials_padding_whitespace }}|", &mut rv).unwrap();

    assert_eq!("|[]|".to_string(), String::from_utf8(rv.into_inner()).unwrap());
}
