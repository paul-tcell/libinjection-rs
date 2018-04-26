use std::os::raw::c_void;
use std::mem::size_of;
use std::slice;
use std::str;

extern {
    fn __assert_fail(__assertion: *const u8, __file: *const u8, __line: u32, __function: *const u8);
    fn memchr(__s: *const c_void, __c: i32, __n: usize) -> *mut c_void;    fn memset(__s: *mut c_void, __c: i32, __n: usize) -> *mut c_void;
    fn strchr(__s: *const u8, __c: i32) -> *mut u8;
}


const CHAR_EOF: i32 = -1;
const CHAR_NULL: i32 = 0;
const CHAR_BANG: i32 = 33;
#[allow(dead_code)]
const CHAR_DOUBLE: i32 = 34;
#[allow(dead_code)]
const CHAR_PERCENT: i32 = 37;
#[allow(dead_code)]
const CHAR_SINGLE: i32 = 39;
const CHAR_DASH: i32 = 45;
#[allow(dead_code)]
const CHAR_SLASH: i32 = 47;
#[allow(dead_code)]
const CHAR_LT: i32 = 60;
#[allow(dead_code)]
const CHAR_EQUALS: i32 = 61;
const CHAR_GT: i32 = 62;
#[allow(dead_code)]
const CHAR_QUESTION: i32 = 63;
#[allow(dead_code)]
const CHAR_RIGHTB: i32 = 93;
#[allow(dead_code)]
const CHAR_TICK: i32 = 96;

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Html5Type {
    DataText,
    TagNameOpen,
    TagNameClose,
    TagNameSelfclose,
    TagData,
    TagClose,
    AttrName,
    AttrValue,
    TagComment,
    DOCTYPE,
}

#[derive(Copy)]
#[repr(C)]
pub struct H5State {
    pub s: *const u8,
    pub len: usize,
    pub num_chars: usize,
    //could be different than number of bytes.
    pub pos: usize,
    pub is_close: i32,
    pub state: unsafe extern fn(*mut H5State) -> i32,
    pub token_start: *const u8,
    pub token_len: usize,
    pub token_type: Html5Type,
}

pub struct H5State2<'b> {
    pub s: &'b str,
    pub len: usize,
    pub num_chars: usize,
    //could be different than number of bytes.
    pub pos: usize,
    pub is_close: i32,
    pub state: unsafe extern fn(*mut H5State) -> i32,
    pub token_start: usize,
    pub token_len: usize,
    pub token_type: Html5Type,
}


impl Clone for H5State {
    fn clone(&self) -> Self { *self }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Html5Flags {
    DataState,
    ValueNoQuote,
    ValueSingleQuote,
    ValueDoubleQuote,
    ValueBackQuote,
}

#[no_mangle]
pub unsafe extern fn libinjection_h5_init(hs: *mut H5State, s: *const u8, len: usize, flags: Html5Flags) {
    memset(hs as (*mut c_void), 0i32, size_of::<H5State>());
    (*hs).s = s;
    (*hs).len = len;
    if flags as (i32) == Html5Flags::ValueBackQuote as (i32) {
        (*hs).state = h5_state_attribute_value_back_quote as (unsafe extern fn(*mut H5State) -> i32);
    } else if flags as (i32) == Html5Flags::ValueDoubleQuote as (i32) {
        (*hs).state = h5_state_attribute_value_double_quote as (unsafe extern fn(*mut H5State) -> i32);
    } else if flags as (i32) == Html5Flags::ValueSingleQuote as (i32) {
        (*hs).state = h5_state_attribute_value_single_quote as (unsafe extern fn(*mut H5State) -> i32);
    } else if flags as (i32) == Html5Flags::ValueNoQuote as (i32) {
        (*hs).state = h5_state_before_attribute_name as (unsafe extern fn(*mut H5State) -> i32);
    } else if flags as (i32) == Html5Flags::DataState as (i32) {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
    }
}

#[no_mangle]
pub unsafe extern fn libinjection_h5_next(hs: *mut H5State) -> i32 {
    if (*hs).state as (*mut c_void) != 0i32 as (*mut c_void) {
        0i32;
    } else {
        __assert_fail((*b"hs->state != NULL\0").as_ptr(), file!().as_ptr(), line!(), (*b"libinjection_h5_next\0").as_ptr());
    }
    ((*hs).state)(hs as (*mut H5State))
}

pub unsafe extern fn h5_state_eof(_hs: *mut H5State) -> i32 {
    0i32
}

unsafe extern fn h5_state_data(hs: *mut H5State) -> i32 {
    let idx: *const u8;
    if (*hs).len >= (*hs).pos {
        0i32;
    } else {
        __assert_fail((*b"hs->len >= hs->pos\0").as_ptr(), file!().as_ptr(), line!(), (*b"h5_state_data\0").as_ptr());
    }
    idx = memchr((*hs).s.offset((*hs).pos as (isize)) as (*const c_void), 60i32, (*hs).len.wrapping_sub((*hs).pos)) as (*const u8);
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::DataText;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
        if (*hs).token_len == 0usize {
            return 0i32;
        }
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_type = Html5Type::DataText;
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
            (*hs).s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(
            (*hs).pos
        );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
            (*hs).s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(
            1usize
        );
        (*hs).state = h5_state_tag_open as (unsafe extern fn(*mut H5State) -> i32);
        if (*hs).token_len == 0usize {
            return h5_state_tag_open(hs);
        }
    }
    1i32
}

unsafe extern fn h5_state_tag_open(hs: *mut H5State) -> i32 {
    let ch: u8;
    if (*hs).pos >= (*hs).len {
        0i32
    } else {
        ch = *(*hs).s.offset((*hs).pos as (isize));
        (if ch as (i32) == 33i32 {
            (*hs).pos = (*hs).pos.wrapping_add(1usize);
            h5_state_markup_declaration_open(hs)
        } else if ch as (i32) == 47i32 {
            (*hs).pos = (*hs).pos.wrapping_add(1usize);
            (*hs).is_close = 1i32;
            h5_state_end_tag_open(hs)
        } else if ch as (i32) == 63i32 {
            (*hs).pos = (*hs).pos.wrapping_add(1usize);
            h5_state_bogus_comment(hs)
        } else if ch as (i32) == 37i32 {
            (*hs).pos = (*hs).pos.wrapping_add(1usize);
            h5_state_bogus_comment2(hs)
        } else if ch as (i32) >= b'a' as (i32) && (ch as (i32) <= b'z' as (i32)) || ch as (i32) >= b'A' as (i32) && (ch as (i32) <= b'Z' as (i32)) {
            h5_state_tag_name(hs)
        } else if ch as (i32) == 0i32 {
            h5_state_tag_name(hs)
        } else if (*hs).pos == 0usize {
            h5_state_data(hs)
        } else {
            (*hs).token_start = (*hs).s.offset((*hs).pos as (isize)).offset(
                -1isize
            );
            (*hs).token_len = 1usize;
            (*hs).token_type = Html5Type::DataText;
            (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
            1i32
        })
    }
}

unsafe extern fn h5_state_end_tag_open(
    hs: *mut H5State
) -> i32 {
    let ch: u8;
    if (*hs).pos >= (*hs).len {
        0i32
    } else {
        ch = *(*hs).s.offset((*hs).pos as (isize));
        (if ch as (i32) == 62i32 {
            h5_state_data(hs)
        } else if ch as (i32) >= b'a' as (i32) && (ch as (i32) <= b'z' as (i32)) || ch as (i32) >= b'A' as (i32) && (ch as (i32) <= b'Z' as (i32)) {
            h5_state_tag_name(hs)
        } else {
            (*hs).is_close = 0i32;
            h5_state_bogus_comment(hs)
        })
    }
}

unsafe extern fn h5_is_white(ch: u8) -> i32 {
    (strchr(
        (*b" \t\n\x0B\x0C\r\0").as_ptr(),
        ch as (i32),
    ) != 0i32 as (*mut c_void) as (*mut u8)) as (i32)
}

unsafe extern fn h5_state_tag_name_close(hs: *mut H5State) -> i32 {
    (*hs).is_close = 0i32;
    (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
    (*hs).token_len = 1usize;
    (*hs).token_type = Html5Type::TagNameClose;
    (*hs).pos = (*hs).pos.wrapping_add(1usize);
    if (*hs).pos < (*hs).len {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
    } else {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
    }
    1i32
}

unsafe extern fn h5_state_tag_name(hs: *mut H5State) -> i32 {
    let current_block;
    let mut ch: u8;
    let mut pos: usize;
    pos = (*hs).pos;
    'loop1: loop {
        if !(pos < (*hs).len) {
            current_block = 2;
            break;
        }
        ch = *(*hs).s.offset(pos as (isize));
        if ch as (i32) == 0i32 {
            pos = pos.wrapping_add(1usize);
        } else {
            if h5_is_white(ch) != 0 {
                current_block = 13;
                break;
            }
            if ch as (i32) == 47i32 {
                current_block = 12;
                break;
            }
            if ch as (i32) == 62i32 {
                current_block = 8;
                break;
            }
            pos = pos.wrapping_add(1usize);
        }
    }
    if current_block == 2 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::TagNameOpen;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
        1i32
    } else if current_block == 8 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        if (*hs).is_close != 0 {
            (*hs).pos = pos.wrapping_add(1usize);
            (*hs).is_close = 0i32;
            (*hs).token_type = Html5Type::TagClose;
            (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
        } else {
            (*hs).pos = pos;
            (*hs).token_type = Html5Type::TagNameOpen;
            (*hs).state = h5_state_tag_name_close as (unsafe extern fn(*mut H5State) -> i32);
        }
        1i32
    } else if current_block == 12 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::TagNameOpen;
        (*hs).pos = pos.wrapping_add(1usize);
        (*hs).state = h5_state_self_closing_start_tag as (unsafe extern fn(*mut H5State) -> i32);
        1i32
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::TagNameOpen;
        (*hs).pos = pos.wrapping_add(1usize);
        (*hs).state = h5_state_before_attribute_name as (unsafe extern fn(*mut H5State) -> i32);
        1i32
    }
}

extern fn h5_skip_white(hs: *mut H5State) -> i32 {
    let hs = unsafe { hs.as_mut() }.unwrap();

    while hs.pos < hs.len {
        let ch = unsafe { hs.s.offset(hs.pos as (isize)) } as usize; //todo!
        match ch {
            0x00 | 0x20 | 0x09 | 0x0A | 0x0B | 0x0C | 0x0D => {
                hs.pos += 1;
            }
            _ => {
                return ch as i32;
            }
        }
    }
    return CHAR_EOF;
}

unsafe extern fn h5_state_before_attribute_name(
    hs: *mut H5State
) -> i32 {
    let ch: i32;
    ch = h5_skip_white(hs);
    if ch == 62i32 {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = 1usize;
        (*hs).token_type = Html5Type::TagNameClose;
        (*hs).pos = (*hs).pos.wrapping_add(1usize);
        1i32
    } else if ch == 47i32 {
        (*hs).pos = (*hs).pos.wrapping_add(1usize);
        h5_state_self_closing_start_tag(hs)
    } else if ch == -1i32 {
        0i32
    } else {
        h5_state_attribute_name(hs)
    }
}

unsafe extern fn h5_state_attribute_name(
    hs: *mut H5State
) -> i32 {
    let current_block;
    let mut ch: u8;
    let mut pos: usize;
    pos = (*hs).pos.wrapping_add(1usize);
    'loop1: loop {
        if !(pos < (*hs).len) {
            current_block = 2;
            break;
        }
        ch = *(*hs).s.offset(pos as (isize));
        if h5_is_white(ch) != 0 {
            current_block = 11;
            break;
        }
        if ch as (i32) == 47i32 {
            current_block = 10;
            break;
        }
        if ch as (i32) == 61i32 {
            current_block = 9;
            break;
        }
        if ch as (i32) == 62i32 {
            current_block = 8;
            break;
        }
        pos = pos.wrapping_add(1usize);
    }
    if current_block == 2 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::AttrName;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).pos = (*hs).len;
        1i32
    } else if current_block == 8 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::AttrName;
        (*hs).state = h5_state_tag_name_close as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).pos = pos;
        1i32
    } else if current_block == 9 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::AttrName;
        (*hs).state = h5_state_before_attribute_value as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).pos = pos.wrapping_add(1usize);
        1i32
    } else if current_block == 10 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::AttrName;
        (*hs).state = h5_state_self_closing_start_tag as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).pos = pos.wrapping_add(1usize);
        1i32
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::AttrName;
        (*hs).state = h5_state_after_attribute_name as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).pos = pos.wrapping_add(1usize);
        1i32
    }
}

unsafe extern fn h5_state_after_attribute_name(
    hs: *mut H5State
) -> i32 {
    let c: i32;
    c = h5_skip_white(hs);
    if c == 62i32 {
        h5_state_tag_name_close(hs)
    } else if c == 61i32 {
        (*hs).pos = (*hs).pos.wrapping_add(1usize);
        h5_state_before_attribute_value(hs)
    } else if c == 47i32 {
        (*hs).pos = (*hs).pos.wrapping_add(1usize);
        h5_state_self_closing_start_tag(hs)
    } else if c == -1i32 {
        0i32
    } else {
        h5_state_attribute_name(hs)
    }
}

unsafe extern fn h5_state_before_attribute_value(
    hs: *mut H5State
) -> i32 {
    let c: i32;
    c = h5_skip_white(hs);
    if c == -1i32 {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
        0i32
    } else if c == 34i32 {
        h5_state_attribute_value_double_quote(hs)
    } else if c == 39i32 {
        h5_state_attribute_value_single_quote(hs)
    } else if c == 96i32 {
        h5_state_attribute_value_back_quote(hs)
    } else {
        h5_state_attribute_value_no_quote(hs)
    }
}

unsafe extern fn h5_state_attribute_value_quote(hs: *mut H5State, qchar: u8) -> i32 {
    let idx: *const u8;
    if (*hs).pos > 0usize {
        (*hs).pos = (*hs).pos.wrapping_add(1usize);
    }
    idx = memchr((*hs).s.offset((*hs).pos as (isize)) as (*const c_void), qchar as (i32), (*hs).len.wrapping_sub((*hs).pos)) as (*const u8);
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::AttrValue;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
            (*hs).s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(
            (*hs).pos
        );
        (*hs).token_type = Html5Type::AttrValue;
        (*hs).state = h5_state_after_attribute_value_quoted_state as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).pos = (*hs).pos.wrapping_add(
            (*hs).token_len.wrapping_add(1usize)
        );
    }
    1i32
}

unsafe extern fn h5_state_attribute_value_double_quote(
    hs: *mut H5State
) -> i32 {
    h5_state_attribute_value_quote(hs, 34u8)
}

unsafe extern fn h5_state_attribute_value_single_quote(
    hs: *mut H5State
) -> i32 {
    h5_state_attribute_value_quote(hs, 39u8)
}

unsafe extern fn h5_state_attribute_value_back_quote(
    hs: *mut H5State
) -> i32 {
    h5_state_attribute_value_quote(hs, 96u8)
}

unsafe extern fn h5_state_attribute_value_no_quote(
    hs: *mut H5State
) -> i32 {
    let current_block;
    let mut ch: u8;
    let mut pos: usize;
    pos = (*hs).pos;
    'loop1: loop {
        if !(pos < (*hs).len) {
            current_block = 2;
            break;
        }
        ch = *(*hs).s.offset(pos as (isize));
        if h5_is_white(ch) != 0 {
            current_block = 7;
            break;
        }
        if ch as (i32) == 62i32 {
            current_block = 6;
            break;
        }
        pos = pos.wrapping_add(1usize);
    }
    if current_block == 2 {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::AttrValue;
        1i32
    } else if current_block == 6 {
        (*hs).token_type = Html5Type::AttrValue;
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).pos = pos;
        (*hs).state = h5_state_tag_name_close as (unsafe extern fn(*mut H5State) -> i32);
        1i32
    } else {
        (*hs).token_type = Html5Type::AttrValue;
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).pos = pos.wrapping_add(1usize);
        (*hs).state = h5_state_before_attribute_name as (unsafe extern fn(*mut H5State) -> i32);
        1i32
    }
}

unsafe extern fn h5_state_after_attribute_value_quoted_state(
    hs: *mut H5State
) -> i32 {
    let ch: u8;
    if (*hs).pos >= (*hs).len {
        0i32
    } else {
        ch = *(*hs).s.offset((*hs).pos as (isize));
        (if h5_is_white(ch) != 0 {
            (*hs).pos = (*hs).pos.wrapping_add(1usize);
            h5_state_before_attribute_name(hs)
        } else if ch as (i32) == 47i32 {
            (*hs).pos = (*hs).pos.wrapping_add(1usize);
            h5_state_self_closing_start_tag(hs)
        } else if ch as (i32) == 62i32 {
            (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
            (*hs).token_len = 1usize;
            (*hs).token_type = Html5Type::TagNameClose;
            (*hs).pos = (*hs).pos.wrapping_add(1usize);
            (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
            1i32
        } else {
            h5_state_before_attribute_name(hs)
        })
    }
}

unsafe extern fn h5_state_self_closing_start_tag(
    hs: *mut H5State
) -> i32 {
    let ch: u8;
    if (*hs).pos >= (*hs).len {
        0i32
    } else {
        ch = *(*hs).s.offset((*hs).pos as (isize));
        (if ch as (i32) == 62i32 {
            if (*hs).pos > 0usize {
                0i32;
            } else {
                __assert_fail(
                    (*b"hs->pos > 0\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"h5_state_self_closing_start_tag\0").as_ptr(),
                );
            }
            (*hs).token_start = (*hs).s.offset((*hs).pos as (isize)).offset(
                -1isize
            );
            (*hs).token_len = 2usize;
            (*hs).token_type = Html5Type::TagNameSelfclose;
            (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
            (*hs).pos = (*hs).pos.wrapping_add(1usize);
            1i32
        } else {
            h5_state_before_attribute_name(hs)
        })
    }
}

unsafe extern fn h5_state_bogus_comment(hs: *mut H5State) -> i32 {
    let idx: *const u8;
    idx = memchr((*hs).s.offset((*hs).pos as (isize)) as (*const c_void), 62i32, (*hs).len.wrapping_sub((*hs).pos)) as (*const u8);
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).pos = (*hs).len;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub((*hs).pos);
        (*hs).pos = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
        (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
    }
    (*hs).token_type = Html5Type::TagComment;
    1i32
}

unsafe extern fn h5_state_bogus_comment2(hs: *mut H5State) -> i32 {
    let current_block;
    let mut idx: *const u8;
    let mut pos: usize;
    pos = (*hs).pos;
    'loop1: loop {
        idx = memchr((*hs).s.offset(pos as (isize)) as (*const c_void), 37i32, (*hs).len.wrapping_sub(pos)) as (*const u8);
        if idx == 0i32 as (*mut c_void) as (*const u8) || idx.offset(1isize) >= (*hs).s.offset((*hs).len as (isize)) {
            current_block = 5;
            break;
        }
        if !(*idx.offset(1isize) as (i32) != 62i32) {
            current_block = 3;
            break;
        }
        pos = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
    }
    if current_block == 3 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub((*hs).pos);
        (*hs).pos = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(2usize);
        (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).token_type = Html5Type::TagComment;
        1i32
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).pos = (*hs).len;
        (*hs).token_type = Html5Type::TagComment;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
        1i32
    }
}

unsafe extern fn h5_state_markup_declaration_open(
    hs: *mut H5State
) -> i32 {
    let remaining: usize;
    remaining = (*hs).len.wrapping_sub((*hs).pos);
    if remaining >= 7usize && (*(*hs).s.offset((*hs).pos.wrapping_add(0usize) as (isize)) as (i32) == b'D' as (i32)
        || *(*hs).s.offset((*hs).pos.wrapping_add(0usize) as (isize)) as (i32) == b'd' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(1usize) as (isize)) as (i32) == b'O' as (i32)
        || *(*hs).s.offset((*hs).pos.wrapping_add(1usize) as (isize)) as (i32) == b'o' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(2usize) as (isize)) as (i32) == b'C' as (i32)
        || *(*hs).s.offset((*hs).pos.wrapping_add(2usize) as (isize)) as (i32) == b'c' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(3usize) as (isize)) as (i32) == b'T' as (i32)
        || *(*hs).s.offset((*hs).pos.wrapping_add(3usize) as (isize)) as (i32) == b't' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(4usize) as (isize)) as (i32) == b'Y' as (i32)
        || *(*hs).s.offset((*hs).pos.wrapping_add(4usize) as (isize)) as (i32) == b'y' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(5usize) as (isize)) as (i32) == b'P' as (i32)
        || *(*hs).s.offset((*hs).pos.wrapping_add(5usize) as (isize)) as (i32) == b'p' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(6usize) as (isize)) as (i32) == b'E' as (i32)
        || *(*hs).s.offset((*hs).pos.wrapping_add(6usize) as (isize)) as (i32) == b'e' as (i32)) {
        h5_state_doctype(hs)
    } else if remaining >= 7usize
        && (*(*hs).s.offset((*hs).pos.wrapping_add(0usize) as (isize)) as (i32) == b'[' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(1usize) as (isize)) as (i32) == b'C' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(2usize) as (isize)) as (i32) == b'D' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(3usize) as (isize)) as (i32) == b'A' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(4usize) as (isize)) as (i32) == b'T' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(5usize) as (isize)) as (i32) == b'A' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(6usize) as (isize)) as (i32) == b'[' as (i32)) {
        (*hs).pos = (*hs).pos.wrapping_add(7usize);
        h5_state_cdata(hs)
    } else if remaining >= 2usize
        && (*(*hs).s.offset((*hs).pos.wrapping_add(0usize) as (isize)) as (i32) == b'-' as (i32))
        && (*(*hs).s.offset((*hs).pos.wrapping_add(1usize) as (isize)) as (i32) == b'-' as (i32)) {
        (*hs).pos = (*hs).pos.wrapping_add(2usize);
        h5_state_comment(hs)
    } else {
        h5_state_bogus_comment(hs)
    }
}

//extern fn h5_state_comment_safe(hs: &H5State) -> bool {true}

extern fn h5_state_comment(hs: *mut H5State) -> i32 {
    let hs = unsafe { hs.as_mut() }.unwrap();
    let s = unsafe { slice::from_raw_parts(hs.s, hs.len) };
    let s = str::from_utf8(s).unwrap();
    let end = hs.len;
    let mut pos = hs.pos;

    let mut offset: usize;

    loop {
        let sub_s = &s[pos..(hs.len - pos)];
        let idx = sub_s.find(CHAR_DASH as u8 as char);

        if idx.is_none() || idx.unwrap() > hs.len - 3 {  //todo
            hs.state = h5_state_eof;  //todo wtf is this fn
            hs.token_start = unsafe { hs.s.offset(hs.pos as isize) };
            hs.token_len = hs.len - hs.pos;
            hs.token_type = Html5Type::TagComment;
            return 1;
        }
        let idx = idx.unwrap();

        offset = 1;

        /* skip all nulls */
        while idx + offset < end && s.bytes().nth(idx + offset).unwrap() == CHAR_NULL as u8 {
            offset += 1;
        }
        if idx + offset == end {
            hs.state = h5_state_eof;  //todo wtf is this fn
            hs.token_start = unsafe { hs.s.offset(hs.pos as isize) };
            hs.token_len = hs.len - hs.pos;
            hs.token_type = Html5Type::TagComment;
            return 1;
        }
        let ch = s.bytes().nth(idx + offset).unwrap();
        if CHAR_DASH != ch as i32 && CHAR_BANG != ch as i32 {
            pos = idx + 1;
            continue;
        }
        offset += 1;
        if idx + offset == end {
            hs.state = h5_state_eof;  //todo wtf is this fn
            hs.token_start = unsafe { hs.s.offset(hs.pos as isize) };
            hs.token_len = hs.len - hs.pos;
            hs.token_type = Html5Type::TagComment;
            return 1;
        }
        let ch = s.bytes().nth(idx + offset).unwrap();
        if CHAR_GT != ch as i32 {
            pos = idx + 1;
            continue;
        }
        offset += 1;

        /* ends in --> or -!> */
        if idx + offset == end {
            hs.token_start = unsafe { hs.s.offset(hs.pos as isize) };
            hs.token_len = idx - hs.pos;
            hs.pos = idx + offset;
            hs.state = h5_state_eof;  //todo wtf is this fn
            hs.token_type = Html5Type::TagComment;
            return 1;
        }
    }
}

unsafe extern fn h5_state_cdata(hs: *mut H5State) -> i32 {
    let current_block;
    let mut idx: *const u8;
    let mut pos: usize;
    pos = (*hs).pos;
    'loop1: loop {
        idx = memchr((*hs).s.offset(pos as (isize)) as (*const c_void), 93i32, (*hs).len.wrapping_sub(pos)) as (*const u8);
        if idx == 0i32 as (*mut c_void) as (*const u8) || idx > (*hs).s.offset((*hs).len as (isize)).offset(-3isize) {
            current_block = 5;
            break;
        }
        if *idx.offset(1isize) as (i32) == 93i32 && (*idx.offset(
            2isize
        ) as (i32) == 62i32) {
            current_block = 4;
            break;
        }
        pos = (((idx as (isize)).wrapping_sub(
            (*hs).s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(
            1usize
        );
    }
    if current_block == 4 {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
            (*hs).s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(
            (*hs).pos
        );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
            (*hs).s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(
            3usize
        );
        (*hs).token_type = Html5Type::DataText;
        1i32
    } else {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = Html5Type::DataText;
        1i32
    }
}

unsafe extern fn h5_state_doctype(hs: *mut H5State) -> i32 {
    let idx: *const u8;
    (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
    (*hs).token_type = Html5Type::DOCTYPE;
    idx = memchr((*hs).s.offset((*hs).pos as (isize)) as (*const c_void), 62i32, (*hs).len.wrapping_sub((*hs).pos)) as (*const u8);
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
    } else {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut H5State) -> i32);
        (*hs).token_len = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub((*hs).pos);
        (*hs).pos = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
    }
    1i32
}
