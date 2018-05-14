use libinjection_html5::{Html5Flags, Html5Type};
use libinjection_html5::libinjection_h5_init_safe;
use libinjection_html5::libinjection_h5_next_safe;

static GS_HEX_DECODE_MAP2: [Option<u8>; 256] = [
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

#[derive(Clone, Copy, PartialEq)]
pub enum Attribute {
    TypeNone,
    TypeBlack,
    TypeAttrUrl,
    TypeStyle,
    TypeAttrIndirect,
}

#[derive(Copy)]
pub struct StringType<'a> {
    pub name: &'a [u8],
    pub atype: Attribute,
}

impl<'a> Clone for StringType<'a> {
    fn clone(&self) -> Self { *self }
}

const X_ACTION: &[u8] = b"action";
const X_ATTRIBUTENAME: &[u8] = b"attributename";
const X_BY: &[u8] = b"by";
const X_BACKGROUND: &[u8] = b"background";
const X_DATAFORMATAS: &[u8] = b"dataformatas";
const X_DATASRC: &[u8] = b"datasrc";
const X_DYNSRC: &[u8] = b"dynsrc";
const X_FILTER: &[u8] = b"filter";
const X_FORMACTION: &[u8] = b"formaction";
const X_FOLDER: &[u8] = b"folder";
const X_FROM: &[u8] = b"from";
const X_HANDLER: &[u8] = b"handler";
const X_HREF: &[u8] = b"href";
const X_LOWSRC: &[u8] = b"lowsrc";
const X_POSTER: &[u8] = b"poster";
const X_SRC: &[u8] = b"src";
const X_STYLE: &[u8] = b"style";
const X_TO: &[u8] = b"to";
const X_VALUES: &[u8] = b"values";
const X_XLINKHREF: &[u8] = b"xlink:href";

const BLACKATTR: [StringType; 20] = [
    StringType { name: X_ACTION, atype: Attribute::TypeAttrUrl }, /* form */
    StringType { name: X_ATTRIBUTENAME, atype: Attribute::TypeAttrIndirect }, /* SVG allow indirection of Attribute names */
    StringType { name: X_BY, atype: Attribute::TypeAttrUrl }, /* SVG */
    StringType { name: X_BACKGROUND, atype: Attribute::TypeAttrUrl }, /* IE6, O11 */
    StringType { name: X_DATAFORMATAS, atype: Attribute::TypeBlack }, /* IE */
    StringType { name: X_DATASRC, atype: Attribute::TypeBlack }, /* IE */
    StringType { name: X_DYNSRC, atype: Attribute::TypeAttrUrl }, /* Obsolete img Attribute */
    StringType { name: X_FILTER, atype: Attribute::TypeStyle }, /* Opera, SVG inline style */
    StringType { name: X_FORMACTION, atype: Attribute::TypeAttrUrl }, /* HTML 5 */
    StringType { name: X_FOLDER, atype: Attribute::TypeAttrUrl }, /* Only on A tags, IE-only */
    StringType { name: X_FROM, atype: Attribute::TypeAttrUrl }, /* SVG */
    StringType { name: X_HANDLER, atype: Attribute::TypeAttrUrl }, /* SVG Tiny, Opera */
    StringType { name: X_HREF, atype: Attribute::TypeAttrUrl },
    StringType { name: X_LOWSRC, atype: Attribute::TypeAttrUrl }, /* Obsolete img Attribute */
    StringType { name: X_POSTER, atype: Attribute::TypeAttrUrl }, /* Opera 10,11 */
    StringType { name: X_SRC, atype: Attribute::TypeAttrUrl },
    StringType { name: X_STYLE, atype: Attribute::TypeStyle },
    StringType { name: X_TO, atype: Attribute::TypeAttrUrl }, /* SVG */
    StringType { name: X_VALUES, atype: Attribute::TypeAttrUrl }, /* SVG */
    StringType { name: X_XLINKHREF, atype: Attribute::TypeAttrUrl },
];

const TAG_APPLET: &'static [u8] = b"applet";
/*    , "audio" */
const TAG_BASE: &[u8] = b"base";
const TAG_COMMENT: &[u8] = b"comment"; /* IE http://html5sec.org/#38 */
const TAG_EMBED: &[u8] = b"embed";
/*   ,  "form" */
const TAG_FRAME: &[u8] = b"frame";
const TAG_FRAMESET: &[u8] = b"frameset";
const TAG_HANDLER: &[u8] = b"handler"; /* Opera SVG, effectively a script tag */
const TAG_IFRAME: &[u8] = b"iframe";
const TAG_IMPORT: &[u8] = b"import";
const TAG_ISINDEX: &[u8] = b"isindex";
const TAG_LINK: &[u8] = b"link";
const TAG_LISTENER: &[u8] = b"listener";
/*    const TAG_MARQUEE: &[u8] = b"marquee";  */
const TAG_META: &[u8] = b"meta";
const TAG_NOSCRIPT: &[u8] = b"noscript";
const TAG_OBJECT: &[u8] = b"object";
const TAG_SCRIPT: &[u8] = b"script";
const TAG_STYLE: &[u8] = b"style";
/*    const TAG_VIDEO: &[u8] = b"VIDEO";  */
const TAG_VMLFRAME: &[u8] = b"vmlframe";
const TAG_XML: &[u8] = b"xml";
const TAG_XSS: &[u8] = b"xss";
const TAG_ZZZZZNOTLISTED: &[u8] = b"zzzzznotlisted";

//static mut BLACKATTR: *mut Stringtype = 0 as ( *mut Stringtype);
const BLACKTAG: [&'static [u8]; 21] = [
    TAG_APPLET,
    /*    TAG_AUDIO */
    TAG_BASE,
    TAG_COMMENT, /* IE http://html5sec.org/#38 */
    TAG_EMBED,
    /*TAG_FORM, */
    TAG_FRAME,
    TAG_FRAMESET,
    TAG_HANDLER, /* Opera SVG, effectively a script tag */
    TAG_IFRAME,
    TAG_IMPORT,
    TAG_ISINDEX,
    TAG_LINK,
    TAG_LISTENER,
    /*    TAG_MARQUEE,  */
    TAG_META,
    TAG_NOSCRIPT,
    TAG_OBJECT,
    TAG_SCRIPT,
    TAG_STYLE,
    /*    TAG_VIDEO,  */
    TAG_VMLFRAME,
    TAG_XML,
    TAG_XSS,
    TAG_ZZZZZNOTLISTED
];

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

#[test]
fn test_streq_ignore_case_ignore_nulls() {
    assert!(streq_ignore_case_ignore_nulls(b"abc", b"abc"));
    assert!(!streq_ignore_case_ignore_nulls(b"abcdef", b"abc"));
    assert!(!streq_ignore_case_ignore_nulls(b"abc", b"abcdef"));


    assert!(streq_ignore_case_ignore_nulls(b"abc", b"a\0\0\0\0\0\0bc"));
    assert!(!streq_ignore_case_ignore_nulls(b"a", b""));
    assert!(!streq_ignore_case_ignore_nulls(b"", b"a"));
}


fn change(sz: &mut usize) {
    *sz = 42;
}


fn html_decode_char_at_safe(src: &[u8], consumed: &mut usize) -> Option<u8> {
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
        if let Some(ch) = GS_HEX_DECODE_MAP2[src[3] as usize] {
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

            if let Some(ch) = GS_HEX_DECODE_MAP2[ch as usize] {
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

fn htmlencode_startswith_safe(a: &[u8], b: &[u8]) -> bool {
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
        let ch = html_decode_char_at_safe(&b[b_pos..b.len()], &mut consumed);
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

const VIEWSOURCE_URL_LC: &[u8] = b"view-source";
const DATA_URL_LC: &[u8] = b"data";
const VBSCRIPT_URL_LC: &[u8] = b"vbscript";
const JAVASCRIPT_URL_LC: &[u8] = b"java";


fn is_black_url_safe(s: &[u8]) -> bool {
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
    htmlencode_startswith_safe(VIEWSOURCE_URL_LC, sub) ||
        htmlencode_startswith_safe(DATA_URL_LC, sub) ||
        htmlencode_startswith_safe(VBSCRIPT_URL_LC, sub) ||
        htmlencode_startswith_safe(JAVASCRIPT_URL_LC, sub)
}

fn is_black_attr_safe(safe_s: &[u8]) -> Attribute {
    if safe_s.len() < 2usize {
        return Attribute::TypeNone;
    }
    if safe_s.len() >= 5usize {
        /* JavaScript on.* */
        if safe_s[0] == 'o' as u8 || safe_s[0] == 'O' as u8 &&
            safe_s[1] == 'n' as u8 || safe_s[1] == 'N' as u8 {
            return Attribute::TypeBlack;
        }

        /* XMLNS can be used to create arbitrary tags */
        if safe_s.eq_ignore_ascii_case(b"XMLNS")
            || safe_s.eq_ignore_ascii_case(b"XLINK") {
            return Attribute::TypeBlack;
        }
    }
    let safe_s = safe_s.to_ascii_lowercase();
    let safe_s = safe_s.as_slice();

    match BLACKATTR.binary_search_by_key(&safe_s, |attr| attr.name) {
        Ok(index) => BLACKATTR[index].atype,
        Err(_) => Attribute::TypeNone
    }
}


fn is_black_tag_safe(safe_s: &[u8]) -> bool {
    if safe_s.len() < 3 { return false; }
    let safe_s = safe_s.to_ascii_lowercase();
    let safe_s = safe_s.as_slice();

    if let Ok(_index) = BLACKTAG.binary_search(&safe_s) {
        return true;
    }
    if safe_s.starts_with(b"svg") || safe_s.starts_with(b"xsl") {
        return true;
    }
    false
}

pub fn libinjection_is_xss_safe(s: &[u8], flags: Html5Flags) -> i32 {
    let mut attr: Attribute = Attribute::TypeNone;
    let mut h5 = libinjection_h5_init_safe(s, flags);
    'loop1: loop {
        if libinjection_h5_next_safe(&mut h5) == 0 {
            return 0;
        }
        if h5.token_type != Html5Type::AttrValue {
            attr = Attribute::TypeNone;
        }

        let sub = &s[h5.token_start..h5.token_start + h5.token_len];
        if h5.token_type == Html5Type::DocType {
            return 1;
        } else if h5.token_type == Html5Type::TagNameOpen {
            if is_black_tag_safe(sub) {
                return 1;
            }
        } else if h5.token_type == Html5Type::AttrName {
            attr = is_black_attr_safe(sub);
        } else if h5.token_type == Html5Type::AttrValue {
            match attr {
                Attribute::TypeBlack => { return 1; }
                Attribute::TypeAttrUrl => {
                    if is_black_url_safe(sub) {
                        return 1;
                    }
                }
                Attribute::TypeStyle => { return 1; }
                Attribute::TypeAttrIndirect => {
                    if is_black_attr_safe(sub) != Attribute::TypeNone {
                        return 1;
                    }
                }
                _ => {}
            }
            attr = Attribute::TypeNone;
        } else if h5.token_type as (i32) == Html5Type::TagComment as (i32) {
            if sub.iter().position(|&b| b == b'`').is_some() {
                return 1;
            }

            /* IE conditional comment */
            if h5.token_len > 3usize {
                if sub[0usize] == b'[' &&
                    (sub[1usize] == b'i' || sub[1usize] == b'I') &&
                    (sub[2usize] == b'f' || sub[2usize] == b'F') {
                    return 1;
                }
                if (sub[0usize] == b'x' || sub[0usize] == b'X') &&
                    (sub[1usize] == b'm' || sub[1usize] == b'M') &&
                    (sub[2usize] == b'l' || sub[2usize] == b'L') {
                    return 1;
                }
            }

            if h5.token_len > 5usize {
                if streq_ignore_case_ignore_nulls(b"import", &s[h5.token_start..6usize]) {
                    return 1;
                }
                if streq_ignore_case_ignore_nulls(b"entity", &s[h5.token_start..6usize]) {
                    return 1;
                }
            }
        }
    }
}


pub fn libinjection_xss_safe(s: &[u8]) -> i32 {
    if libinjection_is_xss_safe(s, Html5Flags::DataState) != 0 {
        1i32
    } else if libinjection_is_xss_safe(s, Html5Flags::ValueNoQuote) != 0 {
        1i32
    } else if libinjection_is_xss_safe(s, Html5Flags::ValueSingleQuote) != 0 {
        1i32
    } else if libinjection_is_xss_safe(s, Html5Flags::ValueDoubleQuote) != 0 {
        1i32
    } else if libinjection_is_xss_safe(s, Html5Flags::ValueBackQuote) != 0 {
        1i32
    } else {
        0i32
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_is_xss_simple() {
        let test_html = "<script>alert(document.domain)</script>";
        let is_xss = libinjection_is_xss_safe(test_html.as_bytes(), Html5Flags::DataState);
        //let is_xss = libinjection_is_xss(test_html.as_ptr() as *const u8, test_html.len(), html5_flags::DataState);

        println!("Is XSS? {}", is_xss == 1);
    }


    #[test]
    fn test_sanity() {
        let mut consumed: usize = 0;
        let encoded = b"&#x56;&#x49;&#x45;&#x57;&#x2D;&#x53;&#x4F;&#x55;&#x52;&#x43;&#x45;";

        let ch = html_decode_char_at_safe(encoded, &mut consumed);
        //let ch = unsafe { html_decode_char_at(encoded.as_ptr(), 66, &mut consumed) };
        println!("{:?} {}", ch, consumed);
    }

    #[test]
    fn test_htmlencoded_starts_with() {
        let upper_encoded = b"&#x56;&#x49;&#x45;&#x57;&#x2D;&#x53;&#x4F;&#x55;&#x52;&#x43;&#x45;";
        assert!(htmlencode_startswith_safe(b"view-source", upper_encoded));
        assert!(htmlencode_startswith_safe(b"v", upper_encoded));
        assert!(htmlencode_startswith_safe(b"view-source", b"VIEW-SOURCE"));
        assert!(htmlencode_startswith_safe(b"view-source", b"VIEW-\0SOURCE"));
        assert!(!htmlencode_startswith_safe(b"view-sourc3", upper_encoded));
        assert!(!htmlencode_startswith_safe(b"view-sourc3", b"\0\0\0"));
        assert!(!htmlencode_startswith_safe(b"view-sourc3", b""));
        //encoded null...
        let upper_encoded = b"&#x56;&#x49;&#x45;&#x57;&#x00;&#x2D;&#x53;&#x4F;&#x55;&#x52;&#x43;&#x45;";
        assert!(htmlencode_startswith_safe(b"view-source", upper_encoded));
        let lower_encoded = b"&#x76;&#x69;&#x65;&#x77;&#x2D;&#x73;&#x6F;&#x75;&#x72;&#x63;&#x65;";
        assert!(htmlencode_startswith_safe(b"view-source", lower_encoded));
    }
}