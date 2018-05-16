use statics::BLACKLISTED_TAGS;
use statics::BLACKLISTED_ATTRIBUTES;
use libinjection_xss::Attribute;


pub struct Node<T: 'static> {
    pub byte: u8,
    pub term: Option<T>,
    pub children: &'static [Node<T>],
}

pub fn is_blacklisted<T>(s: &[u8], root: &'static [Node<T>]) -> &'static Option<T> {
    let mut nodes: &[Node<T>] = root;
    let mut current_node: Option<&'static Node<T>> = None;
    for (i, byte) in s.iter().enumerate() {
        if *byte == 0u8 {
            if s.len() - 1 == i {
                match current_node {
                    Some(node) => { return &node.term; }
                    None => { return &None; }
                }
//                let x = current_node.and_then(|node| node.term);  //fails to compile.  why?
//                return &x
            } else {
                continue;
            }
        }
        current_node = nodes.binary_search_by_key(&byte.to_ascii_lowercase(), |node| node.byte)
            .map(|idx| &nodes[idx]).ok();
        if let Some(node) = current_node {
            if s.len() - 1 == i {
                return &node.term;
            }
            nodes = node.children;
        } else {
            return &None;
        }
    }
    &None
}

pub fn is_tag_blacklisted(s: &[u8]) -> bool {
    is_blacklisted(s, &BLACKLISTED_TAGS).is_some()
}


pub fn is_attr_blacklisted(s: &[u8]) -> &Option<Attribute> {
    is_blacklisted(s, &BLACKLISTED_ATTRIBUTES)
}

#[test]
fn test_are_tags_blacklisted() {
    assert!(is_tag_blacklisted(b"applet\0"));
    assert!(is_tag_blacklisted(b"\0applet"));
    assert!(is_tag_blacklisted(b"applet"));
    assert!(is_tag_blacklisted(b"a\0\0\0\0p\0p\0l\0e\0t"));
    assert!(!is_tag_blacklisted(b"apple"));
    assert!(!is_tag_blacklisted(b"apple\0"));
    assert!(!is_tag_blacklisted(b"ap\0ple"));
    assert!(!is_tag_blacklisted(b"\0\0\0"));
    assert!(!is_tag_blacklisted(b""));

}

#[test]
fn test_are_attribs_blacklisted() {
    assert_eq!(is_attr_blacklisted(b"blahblahblah"), &None);

    assert_eq!(is_attr_blacklisted(b"XLink"), &Some(Attribute::TypeBlack));
    assert_eq!(is_attr_blacklisted(b"xlink"), &Some(Attribute::TypeBlack));
    assert_eq!(is_attr_blacklisted(b"xlink\0"), &Some(Attribute::TypeBlack));
    assert_eq!(is_attr_blacklisted(b"\0xlink"), &Some(Attribute::TypeBlack));
    assert_eq!(is_attr_blacklisted(b"xlink:href"), &Some(Attribute::TypeAttrUrl));

    assert_ne!(is_attr_blacklisted(b"xlinke"), &Some(Attribute::TypeBlack));
    assert_eq!(is_attr_blacklisted(b"xlinke"), &None);

}

//    #[test]
//    fn test_is_black_tag() {
//        let foo = is_black_tag(b"not");
//        assert!(foo.is_none());
//
//        let foo = is_black_tag(b"frame");
//        assert!(foo.unwrap());
//
//        let foo = is_black_tag(b"FrAmE");
//        assert!(foo.unwrap());
//    }