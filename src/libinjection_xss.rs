#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use libinjection_html5::{h5_state_eof, h5_state, html5_flags, html5_type, libinjection_h5_init, libinjection_h5_next};
use libc::{memchr};
use libc::c_void;
use std::slice;


static mut gsHexDecodeMap: [i32; 256] = [
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 256, 256,
256, 256, 256, 256, 256, 10, 11, 12, 13, 14, 15, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 10, 11, 12, 13, 14, 15, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
256, 256, 256, 256
];

#[derive(Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum Attribute {
    TypeNone,
    TypeBlack,
    TypeAttrUrl,
    TypeStyle,
    TypeAttrIndirect,
}

#[derive(Copy)]
#[repr(C)]
pub struct stringtype<'a> {
    pub name: &'a [u8],
    pub atype: Attribute,
}

impl <'a> Clone for stringtype<'a> {
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
const X_NONEY_McNONESTEIN: &[u8]  = b"zzzznone";


const BLACKATTR: [stringtype; 21] = [
    stringtype { name: X_ACTION, atype: Attribute::TypeAttrUrl }, /* form */
    stringtype { name: X_ATTRIBUTENAME, atype: Attribute::TypeAttrIndirect }, /* SVG allow indirection of Attribute names */
    stringtype { name: X_BY, atype: Attribute::TypeAttrUrl }, /* SVG */
    stringtype { name: X_BACKGROUND, atype: Attribute::TypeAttrUrl }, /* IE6, O11 */
    stringtype { name: X_DATAFORMATAS, atype: Attribute::TypeBlack }, /* IE */
    stringtype { name: X_DATASRC, atype: Attribute::TypeBlack }, /* IE */
    stringtype { name: X_DYNSRC, atype: Attribute::TypeAttrUrl }, /* Obsolete img Attribute */
    stringtype { name: X_FILTER, atype: Attribute::TypeStyle }, /* Opera, SVG inline style */
    stringtype { name: X_FORMACTION, atype: Attribute::TypeAttrUrl }, /* HTML 5 */
    stringtype { name: X_FOLDER, atype: Attribute::TypeAttrUrl }, /* Only on A tags, IE-only */
    stringtype { name: X_FROM, atype: Attribute::TypeAttrUrl }, /* SVG */
    stringtype { name: X_HANDLER, atype: Attribute::TypeAttrUrl }, /* SVG Tiny, Opera */
    stringtype { name: X_HREF, atype: Attribute::TypeAttrUrl },
    stringtype { name: X_LOWSRC, atype: Attribute::TypeAttrUrl }, /* Obsolete img Attribute */
    stringtype { name: X_POSTER, atype: Attribute::TypeAttrUrl }, /* Opera 10,11 */
    stringtype { name: X_SRC, atype: Attribute::TypeAttrUrl },
    stringtype { name: X_STYLE, atype: Attribute::TypeStyle },
    stringtype { name: X_TO, atype: Attribute::TypeAttrUrl }, /* SVG */
    stringtype { name: X_VALUES, atype: Attribute::TypeAttrUrl }, /* SVG */
    stringtype { name: X_XLINKHREF, atype: Attribute::TypeAttrUrl },
    stringtype { name: X_NONEY_McNONESTEIN, atype: Attribute::TypeNone },
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


fn cstrcasecmp_with_null(mut a: *const u8, mut b: *const u8, mut n: usize) -> i32 {
    let mut _currentBlock;
    let mut ca : u8;
    let mut cb : u8;
    'loop1: loop {
        if !({
            let _old = n;
            n = n.wrapping_sub(1usize);
            _old
        } > 0usize) {
            _currentBlock = 2;
            break;
        }
        cb = unsafe {
            *{
                let _old = b;
                b = b.offset(1isize);
                _old
            }
        };
        if cb as (i32) == b'\0' as (i32) {
            continue;
        }
        ca = unsafe {
            *{
                let _old = a;
                a = a.offset(1isize);
                _old
            }
        };
        if cb as (i32) >= b'a' as (i32) && (cb as (i32) <= b'z' as (i32)) {
            cb = (cb as (i32) - 0x20i32) as (u8);
        }
        if ca as (i32) != cb as (i32) {
            _currentBlock = 9;
            break;
        }
    }
    if _currentBlock == 2 {
        (if unsafe { *a } as (i32) == 0i32 { 0i32 } else { 1i32 })
    } else {
        1i32
    }
}

unsafe fn html_decode_char_at(mut src: *const u8, mut len: usize, mut consumed: *mut usize) -> i32 {
    let mut _currentBlock;
    let mut val: i32 = 0i32;  //todo unused assignment
    let mut i: usize;
    let mut ch: i32;
    if len == 0usize || src == 0i32 as (*mut c_void) as (*const u8) {
        *consumed = 0usize;
        -1i32
    } else {
        *consumed = 1usize;
        (if *src as (i32) != b'&' as (i32) || len < 2usize {
            *src as (i32)
        } else if *src.offset(1isize) as (i32) != b'#' as (i32) {
            b'&' as (i32)
        } else if *src.offset(2isize) as (i32) == b'x' as (i32) || *src.offset(2isize) as (i32) == b'X' as (i32) {
            ch = *src.offset(3isize) as (i32);
            ch = gsHexDecodeMap[ch as (usize)];
            (if ch == 256i32 {
                b'&' as (i32)
            } else {
                val = ch;
                i = 4usize;
                'loop18: loop {
                    if !(i < len) {
                        _currentBlock = 19;
                        break;
                    }
                    ch = *src.offset(i as (isize)) as (i32);
                    if ch == b';' as (i32) {
                        _currentBlock = 26;
                        break;
                    }
                    ch = gsHexDecodeMap[ch as (usize)];
                    if ch == 256i32 {
                        _currentBlock = 25;
                        break;
                    }
                    val = val * 16i32 + ch;
                    if val > 0x1000ffi32 {
                        _currentBlock = 24;
                        break;
                    }
                    i = i.wrapping_add(1usize);
                }
                (if _currentBlock == 19 {
                    *consumed = i;
                    val
                } else if _currentBlock == 24 {
                    b'&' as (i32)
                } else if _currentBlock == 25 {
                    *consumed = i;
                    val
                } else {
                    *consumed = i.wrapping_add(1usize);
                    val
                })
            })
        } else {
            i = 2usize;
            ch = *src.offset(i as (isize)) as (i32);
            (if ch < b'0' as (i32) || ch > b'9' as (i32) {
                b'&' as (i32)
            } else {
                val = ch - b'0' as (i32);
                i = i.wrapping_add(1usize);
                'loop6: loop {
                    if !(i < len) {
                        _currentBlock = 7;
                        break;
                    }
                    ch = *src.offset(i as (isize)) as (i32);
                    if ch == b';' as (i32) {
                        _currentBlock = 14;
                        break;
                    }
                    if ch < b'0' as (i32) || ch > b'9' as (i32) {
                        _currentBlock = 13;
                        break;
                    }
                    val = val * 10i32 + (ch - b'0' as (i32));
                    if val > 0x1000ffi32 {
                        _currentBlock = 12;
                        break;
                    }
                    i = i.wrapping_add(1usize);
                }
                (if _currentBlock == 7 {
                    *consumed = i;
                    val
                } else if _currentBlock == 12 {
                    b'&' as (i32)
                } else if _currentBlock == 13 {
                    *consumed = i;
                    val
                } else {
                    *consumed = i.wrapping_add(1usize);
                    val
                })
            })
        })
    }
}

fn htmlencode_startswith(mut a: *const u8, mut b: *const u8, mut n: usize) -> i32 {
    let mut _currentBlock;
    let mut consumed: usize = 0; //pmc added =0
    let mut cb: i32;
    let mut first: i32 = 1i32;
    'loop1: loop {
        if !(n > 0usize) {
            _currentBlock = 2;
            break;
        }
        if unsafe { *a } as (i32) == 0i32 {
            _currentBlock = 12;
            break;
        }
        cb = unsafe { html_decode_char_at(b, n, &mut consumed as (*mut usize)) };
        b = unsafe { b.offset(consumed as (isize)) };
        n = n.wrapping_sub(consumed);
        if first != 0 && (cb <= 32i32) {
            continue;
        }
        first = 0i32;
        if cb == 0i32 {
            continue;
        }
        if cb == 10i32 {
            continue;
        }
        if cb >= b'a' as (i32) && (cb <= b'z' as (i32)) {
            cb = cb - 0x20i32;
        }
        if unsafe { *a } as (i32) != cb as (u8) as (i32) {
            _currentBlock = 11;
            break;
        }
        a = unsafe { a.offset(1isize) };
    }
    if _currentBlock == 2 {
        (if unsafe { *a } as (i32) == 0i32 { 1i32 } else { 0i32 })
    } else if _currentBlock == 11 {
        0i32
    } else {
        1i32
    }
}


const VIEWSOURCE_URL: &[u8] = b"VIEW-SOURCE\0";  //todo: do we need these trailing nuls?
const DATA_URL: &[u8] = b"DATA\0";
const VBSCRIPT_URL: &[u8] = b"VBSCRIPT\0";
const JAVASCRIPT_URL: &[u8] = b"JAVA\0";

fn is_black_url(mut s: *const u8, mut len: usize) -> i32 {
    'loop1: loop {
        unsafe {
            if !(len > 0usize && (*s as (i32) <= 32i32 || *s as (i32) >= 127i32)) {
                break;
            }
        }
        s = unsafe { s.offset(1isize) };
        len = len.wrapping_sub(1usize);
    }
    if htmlencode_startswith(DATA_URL.as_ptr(), s, len) != 0 {
        1i32
    } else if htmlencode_startswith(VIEWSOURCE_URL.as_ptr(), s, len) != 0 {
        1i32
    } else if htmlencode_startswith(JAVASCRIPT_URL.as_ptr(), s, len) != 0 {
        1i32
    } else if htmlencode_startswith(VBSCRIPT_URL.as_ptr(), s, len) != 0 {
        1i32
    } else {
        0i32
    }
}

extern fn is_black_attr(mut s: *const u8, mut len: usize) -> Attribute {
    let safe_s = unsafe { slice::from_raw_parts(s, len) };
    if len < 2usize {
        return Attribute::TypeNone;
    }
    if len >= 5usize {
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

extern fn is_black_tag(    mut s : *const u8, mut len : usize) -> i32 {
    if len < 3 { return 0 }
    let safe_s = unsafe { slice::from_raw_parts(s, len) };
    let safe_s = safe_s.to_ascii_lowercase();
    let safe_s = safe_s.as_slice();

    if let Ok(index) = BLACKTAG.binary_search(&safe_s) {
        return 1
    }

    if safe_s.starts_with(b"svg") || safe_s.starts_with(b"xsl") {
        return 1
    }
    0i32
}

#[no_mangle]
pub fn libinjection_is_xss(mut s: *const u8, mut len: usize, mut flags: html5_flags) -> i32 {
    let mut _currentBlock;
    let mut h5 : h5_state = h5_state {
        s: 0 as *const u8,
        len: 0,
        pos: 0,
        is_close: 0,
        state: h5_state_eof,
        token_start: 0 as *const u8,
        token_len: 0,
        token_type: html5_type::TAG_COMMENT,
    };
    let mut attr : Attribute = Attribute::TypeNone;
    libinjection_h5_init(&mut h5 as (*mut h5_state), s, len, flags);
    'loop1: loop {
        if libinjection_h5_next(&mut h5) == 0 {
            _currentBlock = 2;
            break;
        }
        if h5.token_type as (i32) != html5_type::ATTR_VALUE as (i32) {
            attr = Attribute::TypeNone;
        }
        if h5.token_type as (i32) == html5_type::DOCTYPE as (i32) {
            _currentBlock = 37;
            break;
        }
        if h5.token_type as (i32) == html5_type::TAG_NAME_OPEN as (i32) {
            if is_black_tag(h5.token_start,h5.token_len) != 0 {
                _currentBlock = 36;
                break;
            }
        } else if h5.token_type as (i32) == html5_type::ATTR_NAME as (i32) {
            attr = is_black_attr(h5.token_start,h5.token_len);
        } else if h5.token_type as (i32) == html5_type::ATTR_VALUE as (i32) {
            if !(attr as (i32) == Attribute::TypeNone as (i32)) {
                if attr as (i32) == Attribute::TypeAttrIndirect as (i32) {
                    if is_black_attr(h5.token_start,h5.token_len) != Attribute::TypeNone {
                        _currentBlock = 32;
                        break;
                    }
                } else {
                    if attr as (i32) == Attribute::TypeStyle as (i32) {
                        _currentBlock = 30;
                        break;
                    }
                    if attr as (i32) == Attribute::TypeAttrUrl as (i32) {
                        if is_black_url(h5.token_start,h5.token_len) != 0 {
                            _currentBlock = 29;
                            break;
                        }
                    } else if attr as (i32) == Attribute::TypeBlack as (i32) {
                        _currentBlock = 27;
                        break;
                    }
                }
            }
            attr = Attribute::TypeNone;
        } else {
            if !(h5.token_type as (i32) == html5_type::TAG_COMMENT as (i32)) {
                continue;
            }
            if unsafe { memchr(h5.token_start as (*const c_void), b'`' as (i32), h5.token_len) != 0i32 as (*mut c_void) } {
                _currentBlock = 21;
                break;
            }
            unsafe {
                if h5.token_len > 3usize {
                    if *h5.token_start.offset(0isize) as (i32) == b'[' as (i32) &&
                        (*h5.token_start.offset(1isize) as (i32) == b'i' as (i32) || *h5.token_start.offset(1isize) as (i32) == b'I' as (i32)) &&
                        (*h5.token_start.offset(2isize) as (i32) == b'f' as (i32) || *h5.token_start.offset(2isize) as (i32) == b'F' as (i32)) {
                        _currentBlock = 20;
                        break;
                    }
                    if (*h5.token_start.offset(0isize) as (i32) == b'x' as (i32) || *h5.token_start.offset(0isize) as (i32) == b'X' as (i32)) &&
                        (*h5.token_start.offset(1isize) as (i32) == b'm' as (i32) || *h5.token_start.offset(1isize) as (i32) == b'M' as (i32)) &&
                        (*h5.token_start.offset(2isize) as (i32) == b'l' as (i32) || *h5.token_start.offset(2isize) as (i32) == b'L' as (i32)) {
                        _currentBlock = 19;
                        break;
                    }
                }
            }
            if !(h5.token_len > 5usize) {
                continue;
            }
            if cstrcasecmp_with_null((*b"IMPORT\0").as_ptr(), h5.token_start, 6usize) == 0i32 {
                _currentBlock = 18;
                break;
            }
            if cstrcasecmp_with_null((*b"ENTITY\0").as_ptr(), h5.token_start, 6usize) == 0i32 {
                _currentBlock = 17;
                break;
            }
        }
    }
    if _currentBlock == 2 {
        0i32
    } else if _currentBlock == 17 {
        1i32
    } else if _currentBlock == 18 {
        1i32
    } else if _currentBlock == 19 {
        1i32
    } else if _currentBlock == 20 {
        1i32
    } else if _currentBlock == 21 {
        1i32
    } else if _currentBlock == 27 {
        1i32
    } else if _currentBlock == 29 {
        1i32
    } else if _currentBlock == 30 {
        1i32
    } else if _currentBlock == 32 {
        1i32
    } else if _currentBlock == 36 {
        1i32
    } else {
        1i32
    }
}

#[no_mangle]
pub unsafe extern fn libinjection_xss(    mut s : *const u8, mut len : usize) -> i32 {
    if libinjection_is_xss(s, len, html5_flags::DATA_STATE) != 0 {
        1i32
    } else if libinjection_is_xss(s, len, html5_flags::VALUE_NO_QUOTE) != 0 {
        1i32
    } else if libinjection_is_xss(s, len, html5_flags::VALUE_SINGLE_QUOTE) != 0 {
        1i32
    } else if libinjection_is_xss(s, len, html5_flags::VALUE_DOUBLE_QUOTE) != 0 {
        1i32
    } else if libinjection_is_xss(s, len, html5_flags::VALUE_BACK_QUOTE) != 0 {
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
        let is_xss = libinjection_is_xss(test_html.as_ptr() as *const u8, test_html.len(), html5_flags::DATA_STATE);

        println!("Is XSS? {}", is_xss == 1);
    }
}