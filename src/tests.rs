use super::*;

#[test]
fn literal_parser() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(Ok(("", ())), parse_joe.parse("Hello Joe!"));

    assert_eq!(
        Ok((" Hello Robert!", ())),
        parse_joe.parse("Hello Joe! Hello Robert!")
    );
    assert_eq!(Err("Hello Mike!"), parse_joe.parse("Hello Mike!"));
}

#[test]
fn identifier_parser() {
    assert_eq!(
        Ok(("", "i-am-an-identifier".to_string())),
        identifier("i-am-an-identifier")
    );

    assert_eq!(
        Ok((" entirely an identifier", "not".to_string())),
        identifier("not entirely an identifier")
    );

    assert_eq!(
        Err("!not at all an identifier"),
        identifier("!not at all an identifier")
    );
}

#[test]
fn pair_combinator() {
    let tag_opener = pair(match_literal("<"), identifier);
    assert_eq!(
        Ok(("/>", ((), "my-first-element".to_string()))),
        tag_opener.parse("<my-first-element/>")
    );

    assert_eq!(Err("oops"), tag_opener.parse("oops"));
    assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}

#[test]
fn right_combinator() {
    let tag_opener = right(match_literal("<"), identifier);

    assert_eq!(
        Ok(("/>", "my-identifier".to_string())),
        tag_opener.parse("<my-identifier/>")
    );

    assert_eq!(Err("oops"), tag_opener.parse("oops"));
    assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}

#[test]
fn one_or_more_combinator() {
    let parser = one_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Err("ahah"), parser.parse("ahah"));
    assert_eq!(Err(""), parser.parse(""));
}

#[test]
fn zero_or_more_combinator() {
    let parser = zero_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
    assert_eq!(Ok(("", vec![])), parser.parse(""));
}

#[test]
fn any_char_parser() {
    assert_eq!(Ok(("ola", 'h')), any_char("hola"));
    assert_eq!(Err(""), any_char(""));
}

#[test]
fn predicate_combinator() {
    let parser = pred(any_char, |c| *c == 'o');
    assert_eq!(Ok(("rg", 'o')), parser.parse("org"));
    assert_eq!(Err("abc"), parser.parse("abc"));
    assert_eq!(Err(""), parser.parse(""));
}

#[test]
fn quoted_string_parser() {
    let parser = quoted_string();
    assert_eq!(
        Ok(("", "Hello there".to_string())),
        parser.parse("\"Hello there\"")
    );
}

#[test]
fn attributes_parser() {
    assert_eq!(
        Ok((
            "",
            vec![
                ("one".to_string(), "1".to_string()),
                ("two".to_string(), "2".to_string()),
            ]
        )),
        attributes().parse(" one=\"1\" two=\"2\"")
    )
}

#[test]
fn single_element_parser() {
    let input = "<div class=\"float\"/>";
    let expected = Ok((
        "",
        Element {
            name: "div".to_string(),
            attributes: vec![("class".to_string(), "float".to_string())],
            children: vec![],
        },
    ));
    assert_eq!(expected, single_element().parse(input));
}

#[test]
fn empty_parent_element() {
    let doc = r#"<top label="Top">  </top>"#;

    let expected = Ok((
        "",
        Element {
            name: "top".to_string(),
            attributes: vec![(String::from("label"), String::from("Top"))],
            children: vec![],
        },
    ));

    let result = element().parse(doc);
    assert_eq!(expected, result);
}

#[test]
fn xml_parser() {
    let doc = r#"
        <top label="Top">
            <semi-bottom label="Bottom"/>

            <middle>
                <bottom label="Another bottom"/>
            </middle>
        </top>"#;
    let parsed_doc = Element {
        name: "top".to_string(),
        attributes: vec![("label".to_string(), "Top".to_string())],
        children: vec![
            Element {
                name: "semi-bottom".to_string(),
                attributes: vec![("label".to_string(), "Bottom".to_string())],
                children: vec![],
            },
            Element {
                name: "middle".to_string(),
                attributes: vec![],
                children: vec![Element {
                    name: "bottom".to_string(),
                    attributes: vec![("label".to_string(), "Another bottom".to_string())],
                    children: vec![],
                }],
            },
        ],
    };
    assert_eq!(Ok(("", parsed_doc)), element().parse(doc));
}

#[test]
fn mismatched_closing_tag() {
    let doc = r#"
        <top> <bottom/> </middle>"#;
    assert_eq!(Err("</middle>"), element().parse(doc));
}
