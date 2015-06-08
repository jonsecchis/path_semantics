#![deny(missing_docs)]

//! Bootstrapped meta rules for mathematical notation.

extern crate range;
extern crate piston_meta;

use std::path::PathBuf;
use range::Range;

/// Stores information about error occursing when parsing syntax.
pub enum SyntaxError {
    /// An io error occured.
    IoError(std::io::Error),
    /// A meta rule failed when parsing syntax.
    MetaError(PathBuf, String, Range, piston_meta::ParseError),
}

impl From<std::io::Error> for SyntaxError {
    fn from(error: std::io::Error) -> SyntaxError {
        SyntaxError::IoError(error)
    }
}

/// Stores information about mathematical functions.
pub struct Syntax {
    /// The source files.
    pub files: Vec<PathBuf>,
}

impl Syntax {
    /// Parses syntax.
    pub fn new(files: Vec<PathBuf>) -> Result<Syntax, SyntaxError> {
        use std::fs::File;
        use std::io::Read;
        use std::rc::Rc;
        use std::cell::RefCell;
        use piston_meta::*;

        let separators: Rc<String> = Rc::new("()[]{},;:/*+-".into());

        let member_bracket = Rule::Optional(Box::new(Optional {
            debug_id: 100,
            rule: Rule::Sequence(Sequence {
                debug_id: 200,
                args: vec![
                    Rule::Token(Token {
                        debug_id: 300,
                        text: Rc::new("[".into()),
                        inverted: false,
                        property: None,
                    }),
                    Rule::Select(Select {
                        debug_id: 400,
                        args: vec![
                            Rule::Token(Token {
                                debug_id: 500,
                                text: Rc::new(":".into()),
                                inverted: false,
                                property: None,
                            }),
                            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                                debug_id: 600,
                                any_characters: separators.clone(),
                                optional: false,
                                property: None,
                            }),
                        ]
                    }),
                    Rule::Token(Token {
                        debug_id: 700,
                        text: Rc::new("]".into()),
                        inverted: false,
                        property: None,
                    }),
                ],
            })
        }));

        let brackets_rule = Node {
            debug_id: 800,
            name: Rc::new("brackets".into()),
            rule: Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 900,
                optional: true,
                allow_trail: false,
                rule: member_bracket,
                by: Rule::Whitespace(Whitespace {
                    debug_id: 1000,
                    optional: false,
                })
            }))
        };

        let path_rule = Node {
            debug_id: 1100,
            name: Rc::new("path".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 1200,
                args: vec![
                    Rule::Optional(Box::new(Optional {
                        debug_id: 1300,
                        rule: Rule::Token(Token {
                            debug_id: 1400,
                            text: Rc::new("::".into()),
                            inverted: false,
                            property: Some(Rc::new("root".into())),
                        }),
                    })),
                    Rule::SeparatedBy(Box::new(SeparatedBy {
                        debug_id: 1500,
                        optional: false,
                        allow_trail: true,
                        rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                            debug_id: 1600,
                            any_characters: separators.clone(),
                            optional: false,
                            property: Some(Rc::new("name".into())),
                        }),
                        by: Rule::Token(Token {
                            debug_id: 1700,
                            text: Rc::new("::".into()),
                            inverted: false,
                            property: None,
                        }),
                    })),
                ]
            }),
        };

        let arg_rule = Node {
            debug_id: 1800,
            name: Rc::new("arg".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 1900,
                args: vec![
                    Rule::Node(NodeRef::Name(Rc::new("brackets".into()), 0)),
                    Rule::Node(NodeRef::Name(Rc::new("path".into()), 0)),
                    Rule::Node(NodeRef::Name(Rc::new("brackets".into()), 0)),
                    Rule::Optional(Box::new(Optional {
                        debug_id: 2000,
                        rule: Rule::Node(NodeRef::Name(
                            Rc::new("repeated_arguments".into()), 0)),
                    }))
                ]
            })
        };

        let arguments = Node {
            debug_id: 2100,
            name: Rc::new("arguments".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 2200,
                args: vec![
                    Rule::Token(Token {
                        debug_id: 2300,
                        text: Rc::new("(".into()),
                        inverted: false,
                        property: None,
                    }),
                    Rule::Whitespace(Whitespace {
                        debug_id: 2400,
                        optional: true,
                    }),
                    Rule::SeparatedBy(Box::new(SeparatedBy {
                        debug_id: 2500,
                        optional: true,
                        allow_trail: true,
                        rule: Rule::Select(Select {
                            debug_id: 2600,
                            args: vec![
                                Rule::Number(Number {
                                    debug_id: 2700,
                                    allow_underscore: true,
                                    property: None,
                                }),
                                Rule::Text(Text {
                                    debug_id: 2800,
                                    allow_empty: true,
                                    property: None,
                                }),
                                Rule::Node(NodeRef::Name(Rc::new("arguments".into()), 0)),
                                Rule::Node(NodeRef::Name(Rc::new("member_lambda".into()), 0)),
                                Rule::Node(NodeRef::Name(Rc::new("lambda".into()), 0)),
                                Rule::Node(NodeRef::Name(Rc::new("arg".into()), 0)),
                            ]
                        }),
                        by: Rule::Sequence(Sequence {
                            debug_id: 2900,
                            args: vec![
                                Rule::Token(Token {
                                    debug_id: 3000,
                                    text: Rc::new(",".into()),
                                    inverted: false,
                                    property: None,
                                }),
                                Rule::Whitespace(Whitespace {
                                    debug_id: 3100,
                                    optional: false,
                                }),
                            ],
                        }),
                    })),
                    Rule::Whitespace(Whitespace {
                        debug_id: 3200,
                        optional: true,
                    }),
                    Rule::Token(Token {
                        debug_id: 3300,
                        text: Rc::new(")".into()),
                        inverted: false,
                        property: None,
                    }),
                ]
            }),
        };

        let repeated_arguments = Node {
            debug_id: 3400,
            name: Rc::new("repeated_arguments".into()),
            rule: Rule::Repeat(Box::new(Repeat {
                debug_id: 3500,
                optional: false,
                rule: Rule::Node(NodeRef::Name(Rc::new("arguments".into()), 0)),
            }))
        };

        let comment_rule = Node {
            debug_id: 3600,
            name: Rc::new("comment".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 3700,
                args: vec![
                    Rule::Whitespace(Whitespace {
                        debug_id: 22,
                        optional: true,
                    }),
                    Rule::Token(Token {
                        debug_id: 3800,
                        text: Rc::new("//".into()),
                        inverted: false,
                        property: None,
                    }),
                    Rule::UntilAny(UntilAny {
                        debug_id: 3900,
                        any_characters: Rc::new("\n".into()),
                        optional: true,
                        property: None,
                    }),
                ],
            })
        };

        let lambda = Node {
            debug_id: 4000,
            name: Rc::new("lambda".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 4100,
                args: vec![
                    Rule::Whitespace(Whitespace {
                        debug_id: 4200,
                        optional: true,
                    }),
                    Rule::Optional(Box::new(Optional {
                        debug_id: 4300,
                        rule: Rule::Sequence(Sequence {
                            debug_id: 4400,
                            args: vec![
                                Rule::Token(Token {
                                    debug_id: 4500,
                                    text: Rc::new("fn".into()),
                                    inverted: false,
                                    property: None,
                                }),
                                Rule::Whitespace(Whitespace {
                                    debug_id: 4600,
                                    optional: true,
                                }),
                            ]
                        }),
                    })),
                    Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                        debug_id: 4700,
                        any_characters: separators.clone(),
                        optional: true,
                        property: Some(Rc::new("name".into())),
                    }),
                    Rule::Whitespace(Whitespace {
                        debug_id: 4800,
                        optional: true,
                    }),
                    Rule::Node(NodeRef::Name(Rc::new("brackets".into()), 0)),
                    Rule::Node(NodeRef::Name(Rc::new("repeated_arguments".into()), 0)),
                    Rule::Whitespace(Whitespace {
                        debug_id: 4900,
                        optional: false,
                    }),
                    Rule::Token(Token {
                        debug_id: 5000,
                        text: Rc::new("->".into()),
                        inverted: false,
                        property: None,
                    }),
                    Rule::Whitespace(Whitespace {
                        debug_id: 5100,
                        optional: false,
                    }),
                    Rule::Node(NodeRef::Name(Rc::new("arg".into()), 0)),
                    Rule::Whitespace(Whitespace {
                        debug_id: 5200,
                        optional: true,
                    }),
                ]
            })
        };

        let fn_rule = Node {
            debug_id: 5300,
            name: Rc::new("fn".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 5400,
                args: vec![
                    Rule::Optional(Box::new(Optional {
                        debug_id: 5500,
                        rule: Rule::Sequence(Sequence {
                            debug_id: 5600,
                            args: vec![
                                Rule::Token(Token {
                                    debug_id: 5700,
                                    text: Rc::new("pub".into()),
                                    inverted: false,
                                    property: None,
                                }),
                                Rule::Whitespace(Whitespace {
                                    debug_id: 5800,
                                    optional: true,
                                }),
                            ]
                        }),
                    })),
                    Rule::Node(NodeRef::Name(Rc::new("lambda".into()), 0)),
                    Rule::Token(Token {
                        debug_id: 5900,
                        text: Rc::new(";".into()),
                        inverted: false,
                        property: None,
                    }),
                    Rule::Whitespace(Whitespace {
                        debug_id: 6000,
                        optional: true,
                    }),
                    Rule::Optional(Box::new(Optional {
                        debug_id: 6100,
                        rule: Rule::Node(NodeRef::Name(Rc::new("comment".into()), 0))
                    })),
                ]
            })
        };


        let use_rule = Node {
            debug_id: 6200,
            name: Rc::new("use".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 6300,
                args: vec![
                    Rule::Whitespace(Whitespace {
                        debug_id: 6400,
                        optional: true,
                    }),
                    Rule::Optional(Box::new(Optional {
                        debug_id: 6500,
                        rule: Rule::Sequence(Sequence {
                            debug_id: 6600,
                            args: vec![
                                Rule::Token(Token {
                                    debug_id: 6700,
                                    text: Rc::new("pub".into()),
                                    inverted: false,
                                    property: None,
                                }),
                                Rule::Whitespace(Whitespace {
                                    debug_id: 6800,
                                    optional: true,
                                }),
                            ]
                        }),
                    })),
                    Rule::Token(Token {
                        debug_id: 6900,
                        text: Rc::new("use".into()),
                        inverted: false,
                        property: None,
                    }),
                    Rule::Whitespace(Whitespace {
                        debug_id: 7000,
                        optional: false,
                    }),
                    Rule::Node(NodeRef::Name(Rc::new("path".into()), 0)),
                    Rule::Optional(Box::new(Optional {
                        debug_id: 7100,
                        rule: Rule::Token(Token {
                            debug_id: 7200,
                            text: Rc::new("*".into()),
                            inverted: false,
                            property: None,
                        }),
                    })),
                    Rule::Token(Token {
                        debug_id: 7300,
                        text: Rc::new(";".into()),
                        inverted: false,
                        property: None,
                    }),
                ]
            })
        };

        let module = Node {
            debug_id: 7400,
            name: Rc::new("module".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 7500,
                args: vec![
                    Rule::Whitespace(Whitespace {
                        debug_id: 7600,
                        optional: true,
                    }),
                    Rule::Optional(Box::new(Optional {
                        debug_id: 7700,
                        rule: Rule::Sequence(Sequence {
                            debug_id: 7800,
                            args: vec![
                                Rule::Token(Token {
                                    debug_id: 7900,
                                    text: Rc::new("pub".into()),
                                    inverted: false,
                                    property: None,
                                }),
                                Rule::Whitespace(Whitespace {
                                    debug_id: 8000,
                                    optional: true,
                                }),
                            ]
                        }),
                    })),
                    Rule::Token(Token {
                        debug_id: 8100,
                        text: Rc::new("mod".into()),
                        inverted: false,
                        property: None,
                    }),
                    Rule::Whitespace(Whitespace {
                        debug_id: 8200,
                        optional: false,
                    }),
                    Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                        debug_id: 8300,
                        any_characters: separators.clone(),
                        optional: true,
                        property: Some(Rc::new("name".into())),
                    }),
                    Rule::Token(Token {
                        debug_id: 8400,
                        text: Rc::new(";".into()),
                        inverted: false,
                        property: None,
                    }),
                ]
            })
        };

        let member_lambda = Node {
            debug_id: 8500,
            name: Rc::new("member_lambda".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 8600,
                args: vec![
                    Rule::Node(NodeRef::Name(Rc::new("arg".into()), 0)),
                    Rule::Whitespace(Whitespace {
                        debug_id: 8700,
                        optional: true,
                    }),
                    Rule::Token(Token {
                        debug_id: 8800,
                        text: Rc::new(":".into()),
                        inverted: false,
                        property: None,
                    }),
                    Rule::Whitespace(Whitespace {
                        debug_id: 8900,
                        optional: false,
                    }),
                    Rule::Node(NodeRef::Name(Rc::new("arg".into()), 0)),
                ]
            }),
        };

        let member_rule = Node {
            debug_id: 9000,
            name: Rc::new("member".into()),
            rule: Rule::Sequence(Sequence {
                debug_id: 9100,
                args: vec![
                    Rule::Node(NodeRef::Name(Rc::new("member_lambda".into()), 0)),
                    Rule::Token(Token {
                        debug_id: 9200,
                        text: Rc::new(";".into()),
                        inverted: false,
                        property: None,
                    }),
                ]
            }),
        };

        let line_rule = Rule::Select(Select {
            debug_id: 9300,
            args: vec![
            Rule::Node(NodeRef::Name(Rc::new("comment".into()), 0)),
                Rule::Node(NodeRef::Name(Rc::new("use".into()), 0)),
                Rule::Node(NodeRef::Name(Rc::new("module".into()), 0)),
                Rule::Node(NodeRef::Name(Rc::new("member".into()), 0)),
                Rule::Node(NodeRef::Name(Rc::new("fn".into()), 0)),
            ]
        });

        let mut lines_rule = Rule::Lines(Box::new(Lines {
            debug_id: 9400,
            rule: line_rule,
        }));

        let refs: Vec<(Rc<String>, _)> = vec![
            (Rc::new("comment".into()), Rc::new(RefCell::new(comment_rule))),
            (Rc::new("use".into()), Rc::new(RefCell::new(use_rule))),
            (Rc::new("module".into()), Rc::new(RefCell::new(module))),
            (Rc::new("fn".into()), Rc::new(RefCell::new(fn_rule))),
            (Rc::new("lambda".into()), Rc::new(RefCell::new(lambda))),
            (Rc::new("arg".into()), Rc::new(RefCell::new(arg_rule))),
            (Rc::new("member".into()), Rc::new(RefCell::new(member_rule))),
            (Rc::new("member_lambda".into()), Rc::new(RefCell::new(member_lambda))),
            (Rc::new("brackets".into()), Rc::new(RefCell::new(brackets_rule))),
            (Rc::new("arguments".into()), Rc::new(RefCell::new(arguments))),
            (Rc::new("path".into()), Rc::new(RefCell::new(path_rule))),
            (Rc::new("repeated_arguments".into()),
                Rc::new(RefCell::new(repeated_arguments))),
        ];

        lines_rule.update_refs(&refs[..]);

        for file in &files {
            let mut file_h = try!(File::open(file));
            let mut source = String::new();
            try!(file_h.read_to_string(&mut source));

            let chars: Vec<char> = source.chars().collect();
            let offset: usize = 0;
            let chars: &[char] = &chars;
            let mut tokenizer = Tokenizer::new();
            let s = TokenizerState::new();
            let res = lines_rule.parse(&mut tokenizer, &s, &chars, offset);
            match res {
                Ok((ok_range, _s, opt_error)) => {
                    /*
                    println!("TEST tokens");
                    for token in &tokenizer.tokens[s.0..] {
                        println!("{:?}", token.0);
                    }
                    */
                    if let Some((range, err)) = opt_error {
                        if ok_range.next_offset() < chars.len() {
                            return Err(SyntaxError::MetaError(
                                file.into(),
                                source,
                                range,
                                err
                            ));
                        }
                    }
                }
                Err((range, err)) => {
                    return Err(SyntaxError::MetaError(
                        file.into(),
                        source,
                        range,
                        err
                    ));

                }
            }
        }
        Ok(Syntax {
            files: files,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax() {
        if let Err(SyntaxError::MetaError(file, source, range, err))
            = Syntax::new(vec![
                "assets/bool.txt".into(),
                "assets/nat.txt".into(),
                "assets/option.txt".into(),
                "assets/string.rs".into(),
            ]) {
            use piston_meta::*;

            let mut std_err = ParseStdErr::new(&source);
            println!("file: {:?}", file);
            // println!("source {}", source);
            std_err.error(range, err);
            assert!(false);
        }
    }
}
