use libinjection_html5::{Html5Flags, Html5Type};
use libinjection_html5::H5State;
use blacklist;


const VIEWSOURCE_URL_LC: &[u8] = b"view-source";
const DATA_URL_LC: &[u8] = b"data";
const VBSCRIPT_URL_LC: &[u8] = b"vbscript";
const JAVASCRIPT_URL_LC: &[u8] = b"java";

static GS_HEX_DECODE_MAP: [Option<u8>; 256] = [
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    Some(0), Some(1),  Some(2),  Some(3),  Some(4),  Some(5),  Some(6),  Some(7),  Some(8),  Some(9),  None,     None,
    None,    None,     None,     None,     None,     Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None,     None,     None,     None,     None,     None,     None,     None,     None,
    None,    None,     None,     None
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Attribute {
    TypeNone,
    TypeBlack,
    TypeAttrUrl,
    TypeStyle,
    TypeAttrIndirect,
}

fn streq_ignore_case_ignore_nulls(a: &[u8], b: &[u8]) -> bool {
    if b.len() < a.len() {
        return false;
    }
    let mut i = 0;
    for cb in b.iter() {
        if cb == &0u8 {
            continue;
        }
        if i == a.len() || a[i] != cb.to_ascii_lowercase() {
            return false;
        }
        i += 1;
    }
    a.len() == i
}


fn html_decode_char_at(src: &[u8], consumed: &mut usize) -> Option<u8> {
    if src.len() == 0 {
        *consumed = 0;
        return None;
    }

    *consumed = 1;
    if src[0] != b'&' || src.len() < 2 {
        return Some(src[0]);
    }
    if src[1] != b'#' {
        return Some(b'&');
    }

    if src[2] == b'x' || src[2] == b'X' {
        let mut val: u32;
        if let Some(ch) = GS_HEX_DECODE_MAP[src[3] as usize] {
            val = ch as u32;
        } else {
            /* degenerate case  '&#[?]' */
            return Some(b'&');
        }
        let mut i = 4;
        while i < src.len() {
            let ch = src[i];
            if ch == b';' {
                *consumed = i + 1;
                return Some(val as u8);
            }

            if let Some(ch) = GS_HEX_DECODE_MAP[ch as usize] {
                val = val * 16 + ch as u32;
            } else {
                *consumed = i;
                return Some(val as u8);
            }
            if val > 0x1000FF {
                return Some(b'&');
            }
            i += 1;
        }
        *consumed = i;
        return Some(val as u8);
    } else {
        let mut i = 2;
        let ch = src[i];
        if ch < b'0' || ch > b'9' {
            return Some(b'&');
        }
        let mut val: u32 = (ch - b'0') as u32;
        i += 1;
        while i < src.len() {
            let ch = src[i];
            if ch == b';' {
                *consumed = i + 1;
                return Some(val as u8);
            }
            if ch < b'0' || ch > b'9' {
                *consumed = i;
                return Some(val as u8);
            }
            val = (val * 10) + (ch - b'0') as u32;
            if val > 0x1000FF {
                return Some(b'&');
            }
            i += 1;
        }
    }
    None
}

fn htmlencode_startswith(a: &[u8], b: &[u8]) -> bool {
    let mut consumed: usize = 0;
    let mut first = true;

    if b.len() < a.len() {
        return false;
    }
    let mut a_pos = 0usize;
    let mut b_pos = 0usize;
    let mut countdown = b.len();
    while countdown > 0 {
        if a_pos >= a.len() {
            return true;
        }
        let ch = html_decode_char_at(&b[b_pos..b.len()], &mut consumed);
        b_pos += consumed;
        countdown -= consumed;
        if let Some(ch) = ch {
            if first && ch < 32u8 {
                continue;
            }
            first = false;
            if ch == 0u8 {
                continue;
            }
            if ch == 10u8 {
                // vertical tab.  Why?
                continue;
            }
            if a[a_pos] != ch.to_ascii_lowercase() {
                return false;
            }
            a_pos += 1;
        }
    }
    a_pos >= a.len()
}

fn is_blacklisted_url(s: &[u8]) -> bool {
    if s.len() == 0 {
        return false;
    }
    let mut i = 0;
    loop {
        if i == s.len() {  //all whitespace
            return false;
        }
        if s[i] > 32 || s[i] < 127 {
            break;
        }
        i = i + 1;
    }
    let sub = &s[i..s.len()];
    htmlencode_startswith(VIEWSOURCE_URL_LC, sub) ||
        htmlencode_startswith(DATA_URL_LC, sub) ||
        htmlencode_startswith(VBSCRIPT_URL_LC, sub) ||
        htmlencode_startswith(JAVASCRIPT_URL_LC, sub)
}

fn is_attr_blacklisted(safe_s: &[u8]) -> &Attribute {
    if safe_s.len() < 2usize {
        return &Attribute::TypeNone;
    }
    if safe_s.len() >= 5usize {
        /* JavaScript on.* */
        if safe_s[0] == 'o' as u8 || safe_s[0] == 'O' as u8 &&
            safe_s[1] == 'n' as u8 || safe_s[1] == 'N' as u8 {
            return &Attribute::TypeBlack;
        }

        /* XMLNS can be used to create arbitrary tags */
        // ^ moved to statics::BLACKLISTED_ATTRIBUTES
    }
    match blacklist::is_attr_blacklisted(safe_s) {
        Some(att) => att,
        None => &Attribute::TypeNone
    }
}

//future enhancement
#[allow(dead_code)]
enum XSSStatus {
    NoXSS,
    BlacklistedElement,
    BlacklistedAttribute,
    BlacklistedURLInAttribute,
    StyleAttribute,
    Comment,
    Other,
    DocType
}

pub fn xss_check(s: &[u8], flags: Html5Flags) -> bool {
    let mut attr: &Attribute = &Attribute::TypeNone;
    let mut h5 = H5State::new(s, flags);
    'loop1: loop {
        if h5.next() == 0 {
            return false;
        }
        if h5.token_type != Html5Type::AttrValue {
            attr = &Attribute::TypeNone;
        }

        let sub = &s[h5.token_start..h5.token_start + h5.token_len];
        if h5.token_type == Html5Type::DocType {
            return true;
        } else if h5.token_type == Html5Type::TagNameOpen {
            if blacklist::is_tag_blacklisted(sub) {
                return true;
            }
        } else if h5.token_type == Html5Type::AttrName {
            attr = is_attr_blacklisted(sub);
        } else if h5.token_type == Html5Type::AttrValue {
            match attr {
                &Attribute::TypeBlack => { return true; }
                &Attribute::TypeAttrUrl => {
                    if is_blacklisted_url(sub) {
                        return true;
                    }
                }
                &Attribute::TypeStyle => { return true; }
                &Attribute::TypeAttrIndirect => {
                    if is_attr_blacklisted(sub) != &Attribute::TypeNone {
                        return true;
                    }
                }
                _ => {}
            }
            attr = &Attribute::TypeNone;
        } else if h5.token_type  == Html5Type::TagComment  {
            if sub.iter().position(|&b| b == b'`').is_some() {
                return true;
            }

            /* IE conditional comment */
            if h5.token_len > 3usize {
                if sub[0usize] == b'[' &&
                    (sub[1usize] == b'i' || sub[1usize] == b'I') &&
                    (sub[2usize] == b'f' || sub[2usize] == b'F') {
                    return true;
                }
                if (sub[0usize] == b'x' || sub[0usize] == b'X') &&
                    (sub[1usize] == b'm' || sub[1usize] == b'M') &&
                    (sub[2usize] == b'l' || sub[2usize] == b'L') {
                    return true;
                }
            }

            if h5.token_len > 5usize {
                if streq_ignore_case_ignore_nulls(b"import", &s[h5.token_start..6usize]) {
                    return true;
                }
                if streq_ignore_case_ignore_nulls(b"entity", &s[h5.token_start..6usize]) {
                    return true;
                }
            }
        }
    }
}


pub fn is_xss(s: &[u8]) -> bool {
    xss_check(s, Html5Flags::DataState) ||
        xss_check(s, Html5Flags::ValueNoQuote) ||
        xss_check(s, Html5Flags::ValueSingleQuote) ||
        xss_check(s, Html5Flags::ValueDoubleQuote) ||
        xss_check(s, Html5Flags::ValueBackQuote)
}

mod tests {
    #[allow(unused_imports)] //why?
    use super::*;

    #[test]
    fn test_is_xss_simple() {
        let test_html = "<script>alert(document.domain)</script>";
        let is_xss = xss_check(test_html.as_bytes(), Html5Flags::DataState);

        println!("Is XSS? {}", is_xss);

        let test_html = "<ScRipt>Alert(Document.Domain)</ScRipt>";
        let is_xss = xss_check(test_html.as_bytes(), Html5Flags::DataState);
        assert!(is_xss );
    }


    #[test]
    fn test_sanity() {
        let mut consumed: usize = 0;
        let encoded = b"&#x56;&#x49;&#x45;&#x57;&#x2D;&#x53;&#x4F;&#x55;&#x52;&#x43;&#x45;";

        let ch = html_decode_char_at(encoded, &mut consumed);
        println!("{:?} {}", ch, consumed);
    }

    #[test]
    fn test_htmlencoded_starts_with() {
        let upper_encoded = b"&#x56;&#x49;&#x45;&#x57;&#x2D;&#x53;&#x4F;&#x55;&#x52;&#x43;&#x45;";
        assert!(htmlencode_startswith(b"view-source", upper_encoded));
        assert!(htmlencode_startswith(b"v", upper_encoded));
        assert!(htmlencode_startswith(b"view-source", b"VIEW-SOURCE"));
        assert!(htmlencode_startswith(b"view-source", b"VIEW-\0SOURCE"));
        assert!(!htmlencode_startswith(b"view-sourc3", upper_encoded));
        assert!(!htmlencode_startswith(b"view-sourc3", b"\0\0\0"));
        assert!(!htmlencode_startswith(b"view-sourc3", b""));
        //encoded null...
        let upper_encoded = b"&#x56;&#x49;&#x45;&#x57;&#x00;&#x2D;&#x53;&#x4F;&#x55;&#x52;&#x43;&#x45;";
        assert!(htmlencode_startswith(b"view-source", upper_encoded));
        let lower_encoded = b"&#x76;&#x69;&#x65;&#x77;&#x2D;&#x73;&#x6F;&#x75;&#x72;&#x63;&#x65;";
        assert!(htmlencode_startswith(b"view-source", lower_encoded));
    }

    #[test]
    fn test_streq_ignore_case_ignore_nulls() {
        assert!(streq_ignore_case_ignore_nulls(b"abc", b"abc"));
        assert!(!streq_ignore_case_ignore_nulls(b"abcdef", b"abc"));
        assert!(!streq_ignore_case_ignore_nulls(b"abc", b"abcdef"));


        assert!(streq_ignore_case_ignore_nulls(b"abc", b"a\0\0\0\0\0\0bc"));
        assert!(!streq_ignore_case_ignore_nulls(b"a", b""));
        assert!(!streq_ignore_case_ignore_nulls(b"", b"a"));
    }
}