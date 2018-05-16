use blacklist::Node;
use libinjection_xss::Attribute;

// Generated tree of blacklisted tags
// see gen_statics.rs for the names of the strings/bytes
pub static BLACKLISTED_TAGS: [Node<bool>; 14] = [
    Node {
        byte: b'a',
        term: None,
        children: &[
            Node {
                byte: b'p',
                term: None,
                children: &[
                    Node {
                        byte: b'p',
                        term: None,
                        children: &[
                            Node {
                                byte: b'l',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b't',
                                                term: Some(true),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'b',
        term: None,
        children: &[
            Node {
                byte: b'a',
                term: None,
                children: &[
                    Node {
                        byte: b's',
                        term: None,
                        children: &[
                            Node {
                                byte: b'e',
                                term: Some(true),
                                children: &[],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'c',
        term: None,
        children: &[
            Node {
                byte: b'o',
                term: None,
                children: &[
                    Node {
                        byte: b'm',
                        term: None,
                        children: &[
                            Node {
                                byte: b'm',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'n',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b't',
                                                        term: Some(true),
                                                        children: &[],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'e',
        term: None,
        children: &[
            Node {
                byte: b'm',
                term: None,
                children: &[
                    Node {
                        byte: b'b',
                        term: None,
                        children: &[
                            Node {
                                byte: b'e',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'd',
                                        term: Some(true),
                                        children: &[],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'f',
        term: None,
        children: &[
            Node {
                byte: b'r',
                term: None,
                children: &[
                    Node {
                        byte: b'a',
                        term: None,
                        children: &[
                            Node {
                                byte: b'm',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b's',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'e',
                                                        term: None,
                                                        children: &[
                                                            Node {
                                                                byte: b't',
                                                                term: Some(true),
                                                                children: &[],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'h',
        term: None,
        children: &[
            Node {
                byte: b'a',
                term: None,
                children: &[
                    Node {
                        byte: b'n',
                        term: None,
                        children: &[
                            Node {
                                byte: b'd',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'l',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'e',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'r',
                                                        term: Some(true),
                                                        children: &[],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'i',
        term: None,
        children: &[
            Node {
                byte: b'f',
                term: None,
                children: &[
                    Node {
                        byte: b'r',
                        term: None,
                        children: &[
                            Node {
                                byte: b'a',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'm',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'e',
                                                term: Some(true),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b'm',
                term: None,
                children: &[
                    Node {
                        byte: b'p',
                        term: None,
                        children: &[
                            Node {
                                byte: b'o',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'r',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b't',
                                                term: Some(true),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b's',
                term: None,
                children: &[
                    Node {
                        byte: b'i',
                        term: None,
                        children: &[
                            Node {
                                byte: b'n',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'd',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'e',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'x',
                                                        term: Some(true),
                                                        children: &[],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'l',
        term: None,
        children: &[
            Node {
                byte: b'i',
                term: None,
                children: &[
                    Node {
                        byte: b'n',
                        term: None,
                        children: &[
                            Node {
                                byte: b'k',
                                term: Some(true),
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        byte: b's',
                        term: None,
                        children: &[
                            Node {
                                byte: b't',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'n',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'e',
                                                        term: None,
                                                        children: &[
                                                            Node {
                                                                byte: b'r',
                                                                term: Some(true),
                                                                children: &[],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'm',
        term: None,
        children: &[
            Node {
                byte: b'e',
                term: None,
                children: &[
                    Node {
                        byte: b't',
                        term: None,
                        children: &[
                            Node {
                                byte: b'a',
                                term: Some(true),
                                children: &[],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'n',
        term: None,
        children: &[
            Node {
                byte: b'o',
                term: None,
                children: &[
                    Node {
                        byte: b's',
                        term: None,
                        children: &[
                            Node {
                                byte: b'c',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'r',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'i',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'p',
                                                        term: None,
                                                        children: &[
                                                            Node {
                                                                byte: b't',
                                                                term: Some(true),
                                                                children: &[],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'o',
        term: None,
        children: &[
            Node {
                byte: b'b',
                term: None,
                children: &[
                    Node {
                        byte: b'j',
                        term: None,
                        children: &[
                            Node {
                                byte: b'e',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'c',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b't',
                                                term: Some(true),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b's',
        term: None,
        children: &[
            Node {
                byte: b'c',
                term: None,
                children: &[
                    Node {
                        byte: b'r',
                        term: None,
                        children: &[
                            Node {
                                byte: b'i',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'p',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b't',
                                                term: Some(true),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b't',
                term: None,
                children: &[
                    Node {
                        byte: b'y',
                        term: None,
                        children: &[
                            Node {
                                byte: b'l',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: Some(true),
                                        children: &[],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'v',
        term: None,
        children: &[
            Node {
                byte: b'm',
                term: None,
                children: &[
                    Node {
                        byte: b'l',
                        term: None,
                        children: &[
                            Node {
                                byte: b'f',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'r',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'a',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'm',
                                                        term: None,
                                                        children: &[
                                                            Node {
                                                                byte: b'e',
                                                                term: Some(true),
                                                                children: &[],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'x',
        term: None,
        children: &[
            Node {
                byte: b'm',
                term: None,
                children: &[
                    Node {
                        byte: b'l',
                        term: Some(true),
                        children: &[],
                    },
                ],
            },
            Node {
                byte: b's',
                term: None,
                children: &[
                    Node {
                        byte: b's',
                        term: Some(true),
                        children: &[],
                    },
                ],
            },
        ],
    },
];

// Generated tree of blacklisted attributes
// see gen_statics.rs for the names of the strings/bytes
pub static BLACKLISTED_ATTRIBUTES: [Node<Attribute>; 11] = [
    Node {
        byte: b'a',
        term: None,
        children: &[
            Node {
                byte: b'c',
                term: None,
                children: &[
                    Node {
                        byte: b't',
                        term: None,
                        children: &[
                            Node {
                                byte: b'i',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'o',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'n',
                                                term: Some(Attribute::TypeAttrUrl),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b't',
                term: None,
                children: &[
                    Node {
                        byte: b't',
                        term: None,
                        children: &[
                            Node {
                                byte: b'r',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'i',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'b',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'u',
                                                        term: None,
                                                        children: &[
                                                            Node {
                                                                byte: b't',
                                                                term: None,
                                                                children: &[
                                                                    Node {
                                                                        byte: b'e',
                                                                        term: None,
                                                                        children: &[
                                                                            Node {
                                                                                byte: b'n',
                                                                                term: None,
                                                                                children: &[
                                                                                    Node {
                                                                                        byte: b'a',
                                                                                        term: None,
                                                                                        children: &[
                                                                                            Node {
                                                                                                byte: b'm',
                                                                                                term: None,
                                                                                                children: &[
                                                                                                    Node {
                                                                                                        byte: b'e',
                                                                                                        term: Some(Attribute::TypeAttrIndirect),
                                                                                                        children: &[],
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ],
                                                                    },
                                                                ],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'b',
        term: None,
        children: &[
            Node {
                byte: b'a',
                term: None,
                children: &[
                    Node {
                        byte: b'c',
                        term: None,
                        children: &[
                            Node {
                                byte: b'k',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'g',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'r',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'o',
                                                        term: None,
                                                        children: &[
                                                            Node {
                                                                byte: b'u',
                                                                term: None,
                                                                children: &[
                                                                    Node {
                                                                        byte: b'n',
                                                                        term: None,
                                                                        children: &[
                                                                            Node {
                                                                                byte: b'd',
                                                                                term: Some(Attribute::TypeAttrUrl),
                                                                                children: &[],
                                                                            },
                                                                        ],
                                                                    },
                                                                ],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b'y',
                term: Some(Attribute::TypeAttrUrl),
                children: &[],
            },
        ],
    },
    Node {
        byte: b'd',
        term: None,
        children: &[
            Node {
                byte: b'a',
                term: None,
                children: &[
                    Node {
                        byte: b't',
                        term: None,
                        children: &[
                            Node {
                                byte: b'a',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'f',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'o',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'r',
                                                        term: None,
                                                        children: &[
                                                            Node {
                                                                byte: b'm',
                                                                term: None,
                                                                children: &[
                                                                    Node {
                                                                        byte: b'a',
                                                                        term: None,
                                                                        children: &[
                                                                            Node {
                                                                                byte: b't',
                                                                                term: None,
                                                                                children: &[
                                                                                    Node {
                                                                                        byte: b'a',
                                                                                        term: None,
                                                                                        children: &[
                                                                                            Node {
                                                                                                byte: b's',
                                                                                                term: Some(Attribute::TypeBlack),
                                                                                                children: &[],
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ],
                                                                    },
                                                                ],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                    Node {
                                        byte: b's',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'r',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'c',
                                                        term: Some(Attribute::TypeBlack),
                                                        children: &[],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b'y',
                term: None,
                children: &[
                    Node {
                        byte: b'n',
                        term: None,
                        children: &[
                            Node {
                                byte: b's',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'r',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'c',
                                                term: Some(Attribute::TypeAttrUrl),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'f',
        term: None,
        children: &[
            Node {
                byte: b'i',
                term: None,
                children: &[
                    Node {
                        byte: b'l',
                        term: None,
                        children: &[
                            Node {
                                byte: b't',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'r',
                                                term: Some(Attribute::TypeStyle),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b'o',
                term: None,
                children: &[
                    Node {
                        byte: b'l',
                        term: None,
                        children: &[
                            Node {
                                byte: b'd',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'r',
                                                term: Some(Attribute::TypeAttrUrl),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                    Node {
                        byte: b'r',
                        term: None,
                        children: &[
                            Node {
                                byte: b'm',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'a',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'c',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b't',
                                                        term: None,
                                                        children: &[
                                                            Node {
                                                                byte: b'i',
                                                                term: None,
                                                                children: &[
                                                                    Node {
                                                                        byte: b'o',
                                                                        term: None,
                                                                        children: &[
                                                                            Node {
                                                                                byte: b'n',
                                                                                term: Some(Attribute::TypeAttrUrl),
                                                                                children: &[],
                                                                            },
                                                                        ],
                                                                    },
                                                                ],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b'r',
                term: None,
                children: &[
                    Node {
                        byte: b'o',
                        term: None,
                        children: &[
                            Node {
                                byte: b'm',
                                term: Some(Attribute::TypeAttrUrl),
                                children: &[],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'h',
        term: None,
        children: &[
            Node {
                byte: b'a',
                term: None,
                children: &[
                    Node {
                        byte: b'n',
                        term: None,
                        children: &[
                            Node {
                                byte: b'd',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'l',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'e',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'r',
                                                        term: Some(Attribute::TypeAttrUrl),
                                                        children: &[],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b'r',
                term: None,
                children: &[
                    Node {
                        byte: b'e',
                        term: None,
                        children: &[
                            Node {
                                byte: b'f',
                                term: Some(Attribute::TypeAttrUrl),
                                children: &[],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'l',
        term: None,
        children: &[
            Node {
                byte: b'o',
                term: None,
                children: &[
                    Node {
                        byte: b'w',
                        term: None,
                        children: &[
                            Node {
                                byte: b's',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'r',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'c',
                                                term: Some(Attribute::TypeAttrUrl),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'p',
        term: None,
        children: &[
            Node {
                byte: b'o',
                term: None,
                children: &[
                    Node {
                        byte: b's',
                        term: None,
                        children: &[
                            Node {
                                byte: b't',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b'r',
                                                term: Some(Attribute::TypeAttrUrl),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b's',
        term: None,
        children: &[
            Node {
                byte: b'r',
                term: None,
                children: &[
                    Node {
                        byte: b'c',
                        term: Some(Attribute::TypeAttrUrl),
                        children: &[],
                    },
                ],
            },
            Node {
                byte: b't',
                term: None,
                children: &[
                    Node {
                        byte: b'y',
                        term: None,
                        children: &[
                            Node {
                                byte: b'l',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: Some(Attribute::TypeStyle),
                                        children: &[],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b't',
        term: None,
        children: &[
            Node {
                byte: b'o',
                term: Some(Attribute::TypeAttrUrl),
                children: &[],
            },
        ],
    },
    Node {
        byte: b'v',
        term: None,
        children: &[
            Node {
                byte: b'a',
                term: None,
                children: &[
                    Node {
                        byte: b'l',
                        term: None,
                        children: &[
                            Node {
                                byte: b'u',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'e',
                                        term: None,
                                        children: &[
                                            Node {
                                                byte: b's',
                                                term: Some(Attribute::TypeAttrUrl),
                                                children: &[],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Node {
        byte: b'x',
        term: None,
        children: &[
            Node {
                byte: b'l',
                term: None,
                children: &[
                    Node {
                        byte: b'i',
                        term: None,
                        children: &[
                            Node {
                                byte: b'n',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b'k',
                                        term: Some(Attribute::TypeBlack),
                                        children: &[
                                            Node {
                                                byte: b':',
                                                term: None,
                                                children: &[
                                                    Node {
                                                        byte: b'h',
                                                        term: None,
                                                        children: &[
                                                            Node {
                                                                byte: b'r',
                                                                term: None,
                                                                children: &[
                                                                    Node {
                                                                        byte: b'e',
                                                                        term: None,
                                                                        children: &[
                                                                            Node {
                                                                                byte: b'f',
                                                                                term: Some(Attribute::TypeAttrUrl),
                                                                                children: &[],
                                                                            },
                                                                        ],
                                                                    },
                                                                ],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Node {
                byte: b'm',
                term: None,
                children: &[
                    Node {
                        byte: b'l',
                        term: None,
                        children: &[
                            Node {
                                byte: b'n',
                                term: None,
                                children: &[
                                    Node {
                                        byte: b's',
                                        term: Some(Attribute::TypeBlack),
                                        children: &[],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
];
