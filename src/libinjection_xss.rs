use libinjection_html5::Html5Type;
use libinjection_html5::{H5State, Html5Flags};
use libinjection_html5::{h5_state_eof, libinjection_h5_init, libinjection_h5_next};

extern {
    //todo: get rid of
    fn memchr(__s: *const ::std::os::raw::c_void, __c: i32, __n: usize) -> *mut ::std::os::raw::c_void;
}

static mut GS_HEX_DECODE_MAP
: [i32; 256] = [
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
pub struct stringtype {
    pub name: *const u8,
    pub atype: Attribute,
}

impl Clone for stringtype {
    fn clone(&self) -> Self { *self }
}

static mut BLACKATTR: *mut stringtype = 0 as (*mut stringtype);

static mut BLACKTAG: *mut *const u8 = 0 as (*mut *const u8);


//#[derive(Copy)]
//#[repr(C)]
//pub struct H5State {
//    pub s: *const u8,
//    pub len: usize,
//    pub pos: usize,
//    pub is_close: i32,
//    pub state: unsafe extern fn(*mut H5State) -> i32,
//    pub token_start: *const u8,
//    pub token_len: usize,
//    pub token_type: Html5Type,
//}

//impl Clone for H5State {
//    fn clone(&self) -> Self { *self }
//}
//
//#[derive(Clone, Copy)]
//#[repr(i32)]
//pub enum Html5Flags {
//    DataState,
//    ValueNoQuote,
//    ValueSingleQuote,
//    ValueDoubleQuote,
//    ValueBackQuote,
//}

unsafe extern fn cstrcasecmp_with_null(
    mut a: *const u8, mut b: *const u8, mut n: usize,
) -> i32 {
    let mut _current_block;
    let mut ca: u8;
    let mut cb: u8;
    'loop1: loop {
        if !({
            let _old = n;
            n = n.wrapping_sub(1usize);
            _old
        } > 0usize) {
            _current_block = 2;
            break;
        }
        cb = *{
            let _old = b;
            b = b.offset(1isize);
            _old
        };
        if cb as (i32) == b'\0' as (i32) {
            continue;
        }
        ca = *{
            let _old = a;
            a = a.offset(1isize);
            _old
        };
        if cb as (i32) >= b'a' as (i32) && (cb as (i32) <= b'z' as (i32)) {
            cb = (cb as (i32) - 0x20i32) as (u8);
        }
        if ca as (i32) != cb as (i32) {
            _current_block = 9;
            break;
        }
    }
    if _current_block == 2 {
        (if *a as (i32) == 0i32 { 0i32 } else { 1i32 })
    } else {
        1i32
    }
}

unsafe extern fn html_decode_char_at(src: *const u8, len: usize, consumed: *mut usize) -> i32 {
    let mut _current_block;
    let mut val: i32 = 0i32;
    let mut i: usize;
    let mut ch: i32;
    if len == 0usize || src == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        *consumed = 0usize;
        -1i32
    } else {
        *consumed = 1usize;
        (if *src as (i32) != b'&' as (i32) || len < 2usize {
            *src as (i32)
        } else if *src.offset(1isize) as (i32) != b'#' as (i32) {
            b'&' as (i32)
        } else if *src.offset(
            2isize
        ) as (i32) == b'x' as (i32) || *src.offset(
            2isize
        ) as (i32) == b'X' as (i32) {
            ch = *src.offset(3isize) as (i32);
            ch = GS_HEX_DECODE_MAP[ch as (usize)];
            (if ch == 256i32 {
                b'&' as (i32)
            } else {
                val = ch;
                i = 4usize;
                'loop18: loop {
                    if !(i < len) {
                        _current_block = 19;
                        break;
                    }
                    ch = *src.offset(i as (isize)) as (i32);
                    if ch == b';' as (i32) {
                        _current_block = 26;
                        break;
                    }
                    ch = GS_HEX_DECODE_MAP[ch as (usize)];
                    if ch == 256i32 {
                        _current_block = 25;
                        break;
                    }
                    val = val * 16i32 + ch;
                    if val > 0x1000ffi32 {
                        _current_block = 24;
                        break;
                    }
                    i = i.wrapping_add(1usize);
                }
                (if _current_block == 19 {
                    *consumed = i;
                    val
                } else if _current_block == 24 {
                    b'&' as (i32)
                } else if _current_block == 25 {
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
                        _current_block = 7;
                        break;
                    }
                    ch = *src.offset(i as (isize)) as (i32);
                    if ch == b';' as (i32) {
                        _current_block = 14;
                        break;
                    }
                    if ch < b'0' as (i32) || ch > b'9' as (i32) {
                        _current_block = 13;
                        break;
                    }
                    val = val * 10i32 + (ch - b'0' as (i32));
                    if val > 0x1000ffi32 {
                        _current_block = 12;
                        break;
                    }
                    i = i.wrapping_add(1usize);
                }
                (if _current_block == 7 {
                    *consumed = i;
                    val
                } else if _current_block == 12 {
                    b'&' as (i32)
                } else if _current_block == 13 {
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

unsafe extern fn htmlencode_startswith(mut a: *const u8, mut b: *const u8, mut n: usize) -> i32 {
    let mut _current_block;
    let mut consumed: usize = 0;
    let mut cb: i32;
    let mut first: i32 = 1i32;
    'loop1: loop {
        if !(n > 0usize) {
            _current_block = 2;
            break;
        }
        if *a as (i32) == 0i32 {
            _current_block = 12;
            break;
        }
        cb = html_decode_char_at(b, n, &mut consumed as (*mut usize));
        b = b.offset(consumed as (isize));
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
        if *a as (i32) != cb as (u8) as (i32) {
            _current_block = 11;
            break;
        }
        a = a.offset(1isize);
    }
    if _current_block == 2 {
        (if *a as (i32) == 0i32 { 1i32 } else { 0i32 })
    } else if _current_block == 11 {
        0i32
    } else {
        1i32
    }
}


const VIEWSOURCE_URL: &[u8] = b"VIEW-SOURCE\0";
const DATA_URL: &[u8] = b"DATA\0";
const VBSCRIPT_URL: &[u8] = b"VBSCRIPT\0";
const JAVASCRIPT_URL: &[u8] = b"JAVA\0";

unsafe extern fn is_black_url(mut s: *const u8, mut len: usize) -> i32 {
    'loop1: loop {
        if !(len > 0usize && (*s as (i32) <= 32i32 || *s as (i32) >= 127i32)) {
            break;
        }
        s = s.offset(1isize);
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

unsafe extern fn is_black_attr(s: *const u8, len: usize) -> Attribute {
    let mut _current_block;
    let mut black: *mut stringtype;
    if len < 2usize {
        Attribute::TypeNone
    } else {
        if len >= 5usize {
            if (*s.offset(0isize) as (i32) == b'o' as (i32) || *s.offset(
                0isize
            ) as (i32) == b'O' as (i32)) && (*s.offset(
                1isize
            ) as (i32) == b'n' as (i32) || *s.offset(
                1isize
            ) as (i32) == b'N' as (i32)) {
                return Attribute::TypeBlack;
            } else if cstrcasecmp_with_null(
                (*b"XMLNS\0").as_ptr(),
                s,
                5usize,
            ) == 0i32 || cstrcasecmp_with_null(
                (*b"XLINK\0").as_ptr(),
                s,
                5usize,
            ) == 0i32 {
                return Attribute::TypeBlack;
            }
        }
        black = BLACKATTR;
        'loop5: loop {
            if !((*black).name != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8)) {
                _current_block = 6;
                break;
            }
            if cstrcasecmp_with_null((*black).name, s, len) == 0i32 {
                _current_block = 9;
                break;
            }
            black = black.offset(1isize);
        }
        (if _current_block == 6 {
            Attribute::TypeNone
        } else {
            (*black).atype
        })
    }
}

unsafe extern fn is_black_tag(s: *const u8, len: usize) -> i32 {
    let mut _current_block;
    let mut black: *mut *const u8;
    if len < 3usize {
        0i32
    } else {
        black = BLACKTAG;
        'loop2: loop {
            if !(*black != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8)) {
                _current_block = 3;
                break;
            }
            if cstrcasecmp_with_null(*black, s, len) == 0i32 {
                _current_block = 10;
                break;
            }
            black = black.offset(1isize);
        }
        (if _current_block == 3 {
            (if (*s.offset(0isize) as (i32) == b's' as (i32) || *s.offset(
                0isize
            ) as (i32) == b'S' as (i32)) && (*s.offset(
                1isize
            ) as (i32) == b'v' as (i32) || *s.offset(
                1isize
            ) as (i32) == b'V' as (i32)) && (*s.offset(
                2isize
            ) as (i32) == b'g' as (i32) || *s.offset(
                2isize
            ) as (i32) == b'G' as (i32)) {
                1i32
            } else if (*s.offset(
                0isize
            ) as (i32) == b'x' as (i32) || *s.offset(
                0isize
            ) as (i32) == b'X' as (i32)) && (*s.offset(
                1isize
            ) as (i32) == b's' as (i32) || *s.offset(
                1isize
            ) as (i32) == b'S' as (i32)) && (*s.offset(
                2isize
            ) as (i32) == b'l' as (i32) || *s.offset(
                2isize
            ) as (i32) == b'L' as (i32)) {
                1i32
            } else {
                0i32
            })
        } else {
            1i32
        })
    }
}

#[no_mangle]
pub unsafe extern fn libinjection_is_xss(s: *const u8, len: usize, flags: Html5Flags) -> i32 {
    let current_block;
    let mut h5: H5State = H5State {
        s: 0 as *const u8,
        len: 0,
        pos: 0,
        is_close: 0,
        num_chars: 0,
        state: h5_state_eof,
        token_start: 0 as *const u8,
        token_len: 0,
        token_type: Html5Type::TagComment,
    };
    let mut attr: Attribute = Attribute::TypeNone;
    libinjection_h5_init(&mut h5 as (*mut H5State), s, len, flags);
    'loop1: loop {
        if libinjection_h5_next(&mut h5 as (*mut H5State)) == 0 {
            current_block = 2;
            break;
        }
        if h5.token_type as (i32) != Html5Type::AttrValue as (i32) {
            attr = Attribute::TypeNone;
        }
        if h5.token_type as (i32) == Html5Type::DOCTYPE as (i32) {
            current_block = 37;
            break;
        }
        if h5.token_type as (i32) == Html5Type::TagNameOpen as (i32) {
            if is_black_tag(h5.token_start, h5.token_len) != 0 {
                current_block = 36;
                break;
            }
        } else if h5.token_type as (i32) == Html5Type::AttrName as (i32) {
            attr = is_black_attr(h5.token_start, h5.token_len);
        } else if h5.token_type as (i32) == Html5Type::AttrValue as (i32) {
            if !(attr as (i32) == Attribute::TypeNone as (i32)) {
                if attr as (i32) == Attribute::TypeAttrIndirect as (i32) {
                    if is_black_attr(h5.token_start, h5.token_len) != Attribute::TypeNone {
                        current_block = 32;
                        break;
                    }
                } else {
                    if attr as (i32) == Attribute::TypeStyle as (i32) {
                        current_block = 30;
                        break;
                    }
                    if attr as (i32) == Attribute::TypeAttrUrl as (i32) {
                        if is_black_url(h5.token_start, h5.token_len) != 0 {
                            current_block = 29;
                            break;
                        }
                    } else if attr as (i32) == Attribute::TypeBlack as (i32) {
                        current_block = 27;
                        break;
                    }
                }
            }
            attr = Attribute::TypeNone;
        } else {
            if !(h5.token_type as (i32) == Html5Type::TagComment as (i32)) {
                continue;
            }
            if memchr(h5.token_start as (*const ::std::os::raw::c_void), b'`' as (i32), h5.token_len) != 0i32 as (*mut ::std::os::raw::c_void) {
                current_block = 21;
                break;
            }
            if h5.token_len > 3usize {
                if *h5.token_start.offset(0isize) as (i32) == b'[' as (i32)
                    && (*h5.token_start.offset(1isize) as (i32) == b'i' as (i32) || *h5.token_start.offset(1isize) as (i32) == b'I' as (i32))
                    && (*h5.token_start.offset(2isize) as (i32) == b'f' as (i32) || *h5.token_start.offset(2isize) as (i32) == b'F' as (i32)) {
                    current_block = 20;
                    break;
                }
                if (*h5.token_start.offset(0isize) as (i32) == b'x' as (i32) || *h5.token_start.offset(0isize) as (i32) == b'X' as (i32))
                    && (*h5.token_start.offset(1isize) as (i32) == b'm' as (i32) || *h5.token_start.offset(1isize) as (i32) == b'M' as (i32))
                    && (*h5.token_start.offset(2isize) as (i32) == b'l' as (i32) || *h5.token_start.offset(2isize) as (i32) == b'L' as (i32)) {
                    current_block = 19;
                    break;
                }
            }
            if !(h5.token_len > 5usize) {
                continue;
            }
            if cstrcasecmp_with_null((*b"IMPORT\0").as_ptr(), h5.token_start, 6usize) == 0i32 {
                current_block = 18;
                break;
            }
            if cstrcasecmp_with_null((*b"ENTITY\0").as_ptr(), h5.token_start, 6usize) == 0i32 {
                current_block = 17;
                break;
            }
        }
    }
    if current_block == 2 {
        0i32
    } else if current_block == 17 {
        1i32
    } else if current_block == 18 {
        1i32
    } else if current_block == 19 {
        1i32
    } else if current_block == 20 {
        1i32
    } else if current_block == 21 {
        1i32
    } else if current_block == 27 {
        1i32
    } else if current_block == 29 {
        1i32
    } else if current_block == 30 {
        1i32
    } else if current_block == 32 {
        1i32
    } else if current_block == 36 {
        1i32
    } else {
        1i32
    }
}

#[no_mangle]
pub unsafe extern fn libinjection_xss(     s: *const u8,  len: usize,) -> i32 {
    if libinjection_is_xss(
        s,
        len,
        Html5Flags::DataState,
    ) != 0 {
        1i32
    } else if libinjection_is_xss(
        s,
        len,
        Html5Flags::ValueNoQuote,
    ) != 0 {
        1i32
    } else if libinjection_is_xss(
        s,
        len,
        Html5Flags::ValueSingleQuote,
    ) != 0 {
        1i32
    } else if libinjection_is_xss(
        s,
        len,
        Html5Flags::ValueDoubleQuote,
    ) != 0 {
        1i32
    } else if libinjection_is_xss(
        s,
        len,
        Html5Flags::ValueBackQuote,
    ) != 0 {
        1i32
    } else {
        0i32
    }
}
