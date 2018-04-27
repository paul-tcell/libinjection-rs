#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use libc::{memchr, memset, strchr};
use libc::c_void;
use std::mem::size_of;

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


extern {
    fn __assert_fail(__assertion: *const u8, __file: *const u8, __line: u32, __function: *const u8);
    //fn memchr( __s : *const c_void, __c : i32, __n : usize ) -> *mut c_void;
    //fn memset( __s : *mut c_void, __c : i32, __n : usize   ) -> *mut c_void;
    //fn strchr(__s : *const u8, __c : i32) -> *mut u8;
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum html5_type {
    DATA_TEXT,
    TAG_NAME_OPEN,
    TAG_NAME_CLOSE,
    TAG_NAME_SELFCLOSE,
    TAG_DATA,
    TAG_CLOSE,
    ATTR_NAME,
    ATTR_VALUE,
    TAG_COMMENT,
    DOCTYPE,
}

#[derive(Copy)]
#[repr(C)]
pub struct h5_state {
    pub s : *const u8,
    pub len : usize,
    pub pos : usize,
    pub is_close : i32,
    pub state : unsafe extern fn(*mut h5_state) -> i32,
    pub token_start : *const u8,
    pub token_len : usize,
    pub token_type : html5_type,
}

impl Clone for h5_state {
    fn clone(&self) -> Self { *self }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum html5_flags {
    DATA_STATE,
    VALUE_NO_QUOTE,
    VALUE_SINGLE_QUOTE,
    VALUE_DOUBLE_QUOTE,
    VALUE_BACK_QUOTE,
}

#[no_mangle]
pub extern fn libinjection_h5_init(mut hs: *mut h5_state, mut s: *const u8, mut len: usize, mut flags: html5_flags) {
    unsafe { memset(hs as (*mut c_void), 0i32, size_of::<h5_state>()) };
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    hs.s = s;
    hs.len = len;
    if flags as (i32) == html5_flags::VALUE_BACK_QUOTE as (i32) {
        hs.state = h5_state_attribute_value_back_quote as (unsafe extern fn(*mut h5_state) -> i32);
    } else if flags as (i32) == html5_flags::VALUE_DOUBLE_QUOTE as (i32) {
        hs.state = h5_state_attribute_value_double_quote as (unsafe extern fn(*mut h5_state) -> i32);
    } else if flags as (i32) == html5_flags::VALUE_SINGLE_QUOTE as (i32) {
        hs.state = h5_state_attribute_value_single_quote as (unsafe extern fn(*mut h5_state) -> i32);
    } else if flags as (i32) == html5_flags::VALUE_NO_QUOTE as (i32) {
        hs.state = h5_state_before_attribute_name as (unsafe extern fn(*mut h5_state) -> i32);
    } else if flags as (i32) == html5_flags::DATA_STATE as (i32) {
        hs.state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
    }
}

#[no_mangle]
pub extern fn libinjection_h5_next(mut hs: *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    if hs.state as (*mut c_void) != 0i32 as (*mut c_void) {
        0i32;
    } else {
        unsafe {__assert_fail((*b"hs->state != NULL\0").as_ptr(), file!().as_ptr(), line!(), (*b"libinjection_h5_next\0").as_ptr())};
    }
    unsafe {(hs.state)(hs as (*mut h5_state)) }
}

unsafe extern fn h5_state_eof(mut hs : *mut h5_state) -> i32 {
    0i32
}

unsafe extern fn h5_state_data(mut hs : *mut h5_state) -> i32 {
    let mut idx : *const u8;
    if (*hs).len >= (*hs).pos {
        0i32;
    } else {
        __assert_fail((*b"hs->len >= hs->pos\0").as_ptr(), file!().as_ptr(), line!(), (*b"h5_state_data\0").as_ptr());
    }
    idx = memchr((*hs).s.offset((*hs).pos as (isize)) as (*const c_void), 60i32, (*hs).len.wrapping_sub((*hs).pos)) as (*const u8);
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::DATA_TEXT;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        if (*hs).token_len == 0usize {
            return 0i32;
        }
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_type = html5_type::DATA_TEXT;
        (*hs).token_len = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub((*hs).pos);
        (*hs).pos = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
        (*hs).state = h5_state_tag_open as (unsafe extern fn(*mut h5_state) -> i32);
        if (*hs).token_len == 0usize {
            return h5_state_tag_open(hs);
        }
    }
    1i32
}

unsafe extern fn h5_state_tag_open(mut hs : *mut h5_state) -> i32 {
    let mut ch : u8;
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
             (*hs).token_type = html5_type::DATA_TEXT;
             (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
             1i32
         })
    }
}

extern fn h5_state_end_tag_open(mut hs: *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut ch : u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = unsafe { *hs.s.offset(hs.pos as (isize)) };
        (if ch as (i32) == 62i32 {
            unsafe { h5_state_data(hs) }
         } else if ch as (i32) >= b'a' as (i32) && (ch as (i32) <= b'z' as (i32)) || ch as (i32) >= b'A' as (i32) && (ch as (i32) <= b'Z' as (i32)) {
             h5_state_tag_name(hs)
         } else {
             hs.is_close = 0i32;
             h5_state_bogus_comment(hs)
         })
    }
}

extern fn h5_is_white(mut ch : u8) -> i32 {
    unsafe {(strchr((*b" \t\n\x0B\x0C\r\0").as_ptr() as *const i8,   //pmc added as i8
            ch as (i32)) != 0i32 as (*mut c_void) as (*mut i8)) as (i32) }
}

extern fn h5_state_tag_name_close(    mut hs : *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");

    hs.is_close = 0i32;
    hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
    hs.token_len = 1usize;
    hs.token_type = html5_type::TAG_NAME_CLOSE;
    hs.pos = hs.pos.wrapping_add(1usize);
    if hs.pos < hs.len {
        hs.state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
    } else {
        hs.state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
    }
    1i32
}

extern fn h5_state_tag_name(mut hs: *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut _currentBlock;
    let mut ch: u8;
    let mut pos: usize;
    pos = hs.pos;
    'loop1: loop {
        if !(pos < hs.len) {
            _currentBlock = 2;
            break;
        }
        ch = unsafe { *hs.s.offset(pos as (isize)) };
        if ch as (i32) == 0i32 {
            pos = pos.wrapping_add(1usize);
        } else {
            if h5_is_white(ch) != 0 {
                _currentBlock = 13;
                break;
            }
            if ch as (i32) == 47i32 {
                _currentBlock = 12;
                break;
            }
            if ch as (i32) == 62i32 {
                _currentBlock = 8;
                break;
            }
            pos = pos.wrapping_add(1usize);
        }
    }
    if _currentBlock == 2 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::TAG_NAME_OPEN;
        hs.state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    } else if _currentBlock == 8 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        if hs.is_close != 0 {
            hs.pos = pos.wrapping_add(1usize);
            hs.is_close = 0i32;
            hs.token_type = html5_type::TAG_CLOSE;
            hs.state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        } else {
            hs.pos = pos;
            hs.token_type = html5_type::TAG_NAME_OPEN;
            hs.state = h5_state_tag_name_close as (unsafe extern fn(*mut h5_state) -> i32);
        }
        1i32
    } else if _currentBlock == 12 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::TAG_NAME_OPEN;
        hs.pos = pos.wrapping_add(1usize);
        hs.state = h5_state_self_closing_start_tag as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::TAG_NAME_OPEN;
        hs.pos = pos.wrapping_add(1usize);
        hs.state = h5_state_before_attribute_name as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    }
}

#[allow(unused_parens)]
extern fn h5_skip_white(hs: *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    while(hs.pos < hs.len) {
        let ch = unsafe { *hs.s.offset(hs.pos as (isize)) }; //todo!
        match ch {
            0x00 | 0x20 | 0x09 | 0x0A | 0x0B | 0x0C | 0x0D => {
                hs.pos = hs.pos.wrapping_add(1usize);
                // or
                // hs.pos += 1;
            }
            _ => {
                return ch as i32;
            }
        }
    }
    return CHAR_EOF;
}


extern fn h5_state_before_attribute_name(    mut hs : *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut ch : i32;
    ch = h5_skip_white(hs);
    if ch == 62i32 {
        hs.state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = 1usize;
        hs.token_type = html5_type::TAG_NAME_CLOSE;
        hs.pos = hs.pos.wrapping_add(1usize);
        1i32
    } else if ch == 47i32 {
        hs.pos = hs.pos.wrapping_add(1usize);
        h5_state_self_closing_start_tag(hs)
    } else if ch == -1i32 {
        0i32
    } else {
        h5_state_attribute_name(hs)
    }
}

extern fn h5_state_attribute_name(mut hs: *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");

    let mut _currentBlock;
    let mut ch: u8;
    let mut pos: usize;
    pos = hs.pos.wrapping_add(1usize);
    'loop1: loop {
        if !(pos < hs.len) {
            _currentBlock = 2;
            break;
        }
        ch = unsafe { *hs.s.offset(pos as (isize)) };
        if h5_is_white(ch) != 0 {
            _currentBlock = 11;
            break;
        }
        if ch as (i32) == 47i32 {
            _currentBlock = 10;
            break;
        }
        if ch as (i32) == 61i32 {
            _currentBlock = 9;
            break;
        }
        if ch as (i32) == 62i32 {
            _currentBlock = 8;
            break;
        }
        pos = pos.wrapping_add(1usize);
    }
    if _currentBlock == 2 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_NAME;
        hs.state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        hs.pos = hs.len;
        1i32
    } else if _currentBlock == 8 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_NAME;
        hs.state = h5_state_tag_name_close as (unsafe extern fn(*mut h5_state) -> i32);
        hs.pos = pos;
        1i32
    } else if _currentBlock == 9 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_NAME;
        hs.state = h5_state_before_attribute_value as (unsafe extern fn(*mut h5_state) -> i32);
        hs.pos = pos.wrapping_add(1usize);
        1i32
    } else if _currentBlock == 10 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_NAME;
        hs.state = h5_state_self_closing_start_tag as (unsafe extern fn(*mut h5_state) -> i32);
        hs.pos = pos.wrapping_add(1usize);
        1i32
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_NAME;
        hs.state = h5_state_after_attribute_name as (unsafe extern fn(*mut h5_state) -> i32);
        hs.pos = pos.wrapping_add(1usize);
        1i32
    }
}

extern fn h5_state_after_attribute_name(mut hs: *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut c: i32;
    c = h5_skip_white(hs);
    if c == 62i32 {
        h5_state_tag_name_close(hs)
    } else if c == 61i32 {
        hs.pos = hs.pos.wrapping_add(1usize);
        h5_state_before_attribute_value(hs)
    } else if c == 47i32 {
        hs.pos = hs.pos.wrapping_add(1usize);
        h5_state_self_closing_start_tag(hs)
    } else if c == -1i32 {
        0i32
    } else {
        h5_state_attribute_name(hs)
    }
}

extern fn h5_state_before_attribute_value(    mut hs : *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut c : i32;
    c = h5_skip_white(hs);
    if c == -1i32 {
        hs.state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
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

extern fn h5_state_attribute_value_quote(    mut hs : *mut h5_state, mut qchar : u8) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut idx : *const u8;
    if hs.pos > 0usize {
        hs.pos = hs.pos.wrapping_add(1usize);
    }
    idx = unsafe { memchr(hs.s.offset(hs.pos as (isize)) as (*const c_void), qchar as (i32), hs.len.wrapping_sub(hs.pos)) as (*const u8) };
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_VALUE;
        hs.state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(hs.pos);        hs.token_type = html5_type::ATTR_VALUE;
        hs.state = h5_state_after_attribute_value_quoted_state as (unsafe extern fn(*mut h5_state) -> i32);
        hs.pos = hs.pos.wrapping_add(hs.token_len.wrapping_add(1usize));
    }
    1i32
}

extern fn h5_state_attribute_value_double_quote(    mut hs : *mut h5_state) -> i32 {
    h5_state_attribute_value_quote(hs,34u8)
}

extern fn h5_state_attribute_value_single_quote(    mut hs : *mut h5_state) -> i32 {
    h5_state_attribute_value_quote(hs,39u8)
}

extern fn h5_state_attribute_value_back_quote(mut hs: *mut h5_state) -> i32 {
    h5_state_attribute_value_quote(hs, 96u8)
}

extern fn h5_state_attribute_value_no_quote(    mut hs : *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut _currentBlock;
    let mut ch : u8;
    let mut pos : usize;
    pos = hs.pos;
    'loop1: loop {
        if !(pos < hs.len) {
            _currentBlock = 2;
            break;
        }
        ch = unsafe { *hs.s.offset(pos as (isize)) };
        if h5_is_white(ch) != 0 {
            _currentBlock = 7;
            break;
        }
        if ch as (i32) == 62i32 {
            _currentBlock = 6;
            break;
        }
        pos = pos.wrapping_add(1usize);
    }
    if _currentBlock == 2 {
        hs.state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_VALUE;
        1i32
    } else if _currentBlock == 6 {
        hs.token_type = html5_type::ATTR_VALUE;
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.pos = pos;
        hs.state = h5_state_tag_name_close as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    } else {
        hs.token_type = html5_type::ATTR_VALUE;
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.pos = pos.wrapping_add(1usize);
        hs.state = h5_state_before_attribute_name as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    }
}

extern fn h5_state_after_attribute_value_quoted_state(    mut hs : *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut ch : u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = unsafe { *hs.s.offset(hs.pos as (isize)) };
        (if h5_is_white(ch) != 0 {
             hs.pos = hs.pos.wrapping_add(1usize);
             h5_state_before_attribute_name(hs)
         } else if ch as (i32) == 47i32 {
             hs.pos = hs.pos.wrapping_add(1usize);
             h5_state_self_closing_start_tag(hs)
         } else if ch as (i32) == 62i32 {
             hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
             hs.token_len = 1usize;
             hs.token_type = html5_type::TAG_NAME_CLOSE;
             hs.pos = hs.pos.wrapping_add(1usize);
             hs.state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
             1i32
         } else {
             h5_state_before_attribute_name(hs)
         })
    }
}

extern fn h5_state_self_closing_start_tag(    mut hs : *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut ch : u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = unsafe { *hs.s.offset(hs.pos as (isize)) };
        (if ch as (i32) == 62i32 {
             if hs.pos > 0usize {
                 0i32;
             } else {
                 unsafe { __assert_fail((*b"hs->pos > 0\0").as_ptr(), file!().as_ptr(), line!(), (*b"h5_state_self_closing_start_tag\0").as_ptr())};
             }
            hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)).offset(-1isize) };
             hs.token_len = 2usize;
             hs.token_type = html5_type::TAG_NAME_SELFCLOSE;
             hs.state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
             hs.pos = hs.pos.wrapping_add(1usize);
             1i32
         } else {
             h5_state_before_attribute_name(hs)
         })
    }
}

extern fn h5_state_bogus_comment(    mut hs : *mut h5_state) -> i32 {
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    let mut idx : *const u8;
    idx = unsafe {memchr(hs.s.offset(hs.pos as (isize)) as (*const c_void), 62i32, hs.len.wrapping_sub(hs.pos)) as (*const u8)};
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.pos = hs.len;
        hs.state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(hs.pos);
        hs.pos = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
        hs.state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
    }
    hs.token_type = html5_type::TAG_COMMENT;
    1i32
}

unsafe extern fn h5_state_bogus_comment2(
    mut hs : *mut h5_state
) -> i32 {
    let mut _currentBlock;
    let mut idx : *const u8;
    let mut pos : usize;
    pos = (*hs).pos;
    'loop1: loop {
        idx = memchr(
                  (*hs).s.offset(pos as (isize)) as (*const c_void),
                  37i32,
                  (*hs).len.wrapping_sub(pos)
              ) as (*const u8);
        if idx == 0i32 as (*mut c_void) as (*const u8) || idx.offset(
                                                                              1isize
                                                                          ) >= (*hs).s.offset(
                                                                                   (*hs).len as (isize)
                                                                               ) {
            _currentBlock = 5;
            break;
        }
        if !(*idx.offset(1isize) as (i32) != 62i32) {
            _currentBlock = 3;
            break;
        }
        pos = (((idx as (isize)).wrapping_sub(
                    (*hs).s as (isize)
                ) / size_of::<u8>(
                    ) as (isize)) as (usize)).wrapping_add(
                  1usize
              );
    }
    if _currentBlock == 3 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
                          (*hs).s as (isize)
                      ) / size_of::<u8>(
                          ) as (isize)) as (usize)).wrapping_add(
                        2usize
                    );
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_type = html5_type::TAG_COMMENT;
        1i32
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).pos = (*hs).len;
        (*hs).token_type = html5_type::TAG_COMMENT;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    }
}

unsafe extern fn h5_state_markup_declaration_open(
    mut hs : *mut h5_state
) -> i32 {
    let mut remaining : usize;
    remaining = (*hs).len.wrapping_sub((*hs).pos);
    if remaining >= 7usize && (*(*hs).s.offset(
                                    (*hs).pos.wrapping_add(0usize) as (isize)
                                ) as (i32) == b'D' as (i32) || *(*hs).s.offset(
                                                                    (*hs).pos.wrapping_add(
                                                                        0usize
                                                                    ) as (isize)
                                                                ) as (i32) == b'd' as (i32)) && (*(*hs).s.offset(
                                                                                                      (*hs).pos.wrapping_add(
                                                                                                          1usize
                                                                                                      ) as (isize)
                                                                                                  ) as (i32) == b'O' as (i32) || *(*hs).s.offset(
                                                                                                                                      (*hs).pos.wrapping_add(
                                                                                                                                          1usize
                                                                                                                                      ) as (isize)
                                                                                                                                  ) as (i32) == b'o' as (i32)) && (*(*hs).s.offset(
                                                                                                                                                                        (*hs).pos.wrapping_add(
                                                                                                                                                                            2usize
                                                                                                                                                                        ) as (isize)
                                                                                                                                                                    ) as (i32) == b'C' as (i32) || *(*hs).s.offset(
                                                                                                                                                                                                        (*hs).pos.wrapping_add(
                                                                                                                                                                                                            2usize
                                                                                                                                                                                                        ) as (isize)
                                                                                                                                                                                                    ) as (i32) == b'c' as (i32)) && (*(*hs).s.offset(
                                                                                                                                                                                                                                          (*hs).pos.wrapping_add(
                                                                                                                                                                                                                                              3usize
                                                                                                                                                                                                                                          ) as (isize)
                                                                                                                                                                                                                                      ) as (i32) == b'T' as (i32) || *(*hs).s.offset(
                                                                                                                                                                                                                                                                          (*hs).pos.wrapping_add(
                                                                                                                                                                                                                                                                              3usize
                                                                                                                                                                                                                                                                          ) as (isize)
                                                                                                                                                                                                                                                                      ) as (i32) == b't' as (i32)) && (*(*hs).s.offset(
                                                                                                                                                                                                                                                                                                            (*hs).pos.wrapping_add(
                                                                                                                                                                                                                                                                                                                4usize
                                                                                                                                                                                                                                                                                                            ) as (isize)
                                                                                                                                                                                                                                                                                                        ) as (i32) == b'Y' as (i32) || *(*hs).s.offset(
                                                                                                                                                                                                                                                                                                                                            (*hs).pos.wrapping_add(
                                                                                                                                                                                                                                                                                                                                                4usize
                                                                                                                                                                                                                                                                                                                                            ) as (isize)
                                                                                                                                                                                                                                                                                                                                        ) as (i32) == b'y' as (i32)) && (*(*hs).s.offset(
                                                                                                                                                                                                                                                                                                                                                                              (*hs).pos.wrapping_add(
                                                                                                                                                                                                                                                                                                                                                                                  5usize
                                                                                                                                                                                                                                                                                                                                                                              ) as (isize)
                                                                                                                                                                                                                                                                                                                                                                          ) as (i32) == b'P' as (i32) || *(*hs).s.offset(
                                                                                                                                                                                                                                                                                                                                                                                                              (*hs).pos.wrapping_add(
                                                                                                                                                                                                                                                                                                                                                                                                                  5usize
                                                                                                                                                                                                                                                                                                                                                                                                              ) as (isize)
                                                                                                                                                                                                                                                                                                                                                                                                          ) as (i32) == b'p' as (i32)) && (*(*hs).s.offset(
                                                                                                                                                                                                                                                                                                                                                                                                                                                (*hs).pos.wrapping_add(
                                                                                                                                                                                                                                                                                                                                                                                                                                                    6usize
                                                                                                                                                                                                                                                                                                                                                                                                                                                ) as (isize)
                                                                                                                                                                                                                                                                                                                                                                                                                                            ) as (i32) == b'E' as (i32) || *(*hs).s.offset(
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                (*hs).pos.wrapping_add(
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    6usize
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                ) as (isize)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            ) as (i32) == b'e' as (i32)) {
        h5_state_doctype(hs)
    } else if remaining >= 7usize && (*(*hs).s.offset(
                                           (*hs).pos.wrapping_add(0usize) as (isize)
                                       ) as (i32) == b'[' as (i32)) && (*(*hs).s.offset(
                                                                             (*hs).pos.wrapping_add(
                                                                                 1usize
                                                                             ) as (isize)
                                                                         ) as (i32) == b'C' as (i32)) && (*(*hs).s.offset(
                                                                                                               (*hs).pos.wrapping_add(
                                                                                                                   2usize
                                                                                                               ) as (isize)
                                                                                                           ) as (i32) == b'D' as (i32)) && (*(*hs).s.offset(
                                                                                                                                                 (*hs).pos.wrapping_add(
                                                                                                                                                     3usize
                                                                                                                                                 ) as (isize)
                                                                                                                                             ) as (i32) == b'A' as (i32)) && (*(*hs).s.offset(
                                                                                                                                                                                   (*hs).pos.wrapping_add(
                                                                                                                                                                                       4usize
                                                                                                                                                                                   ) as (isize)
                                                                                                                                                                               ) as (i32) == b'T' as (i32)) && (*(*hs).s.offset(
                                                                                                                                                                                                                     (*hs).pos.wrapping_add(
                                                                                                                                                                                                                         5usize
                                                                                                                                                                                                                     ) as (isize)
                                                                                                                                                                                                                 ) as (i32) == b'A' as (i32)) && (*(*hs).s.offset(
                                                                                                                                                                                                                                                       (*hs).pos.wrapping_add(
                                                                                                                                                                                                                                                           6usize
                                                                                                                                                                                                                                                       ) as (isize)
                                                                                                                                                                                                                                                   ) as (i32) == b'[' as (i32)) {
        (*hs).pos = (*hs).pos.wrapping_add(7usize);
        h5_state_cdata(hs)
    } else if remaining >= 2usize && (*(*hs).s.offset(
                                           (*hs).pos.wrapping_add(0usize) as (isize)
                                       ) as (i32) == b'-' as (i32)) && (*(*hs).s.offset(
                                                                             (*hs).pos.wrapping_add(
                                                                                 1usize
                                                                             ) as (isize)
                                                                         ) as (i32) == b'-' as (i32)) {
        (*hs).pos = (*hs).pos.wrapping_add(2usize);
        h5_state_comment(hs)
    } else {
        h5_state_bogus_comment(hs)
    }
}

//pmc - rewrote - h5_state_comment_see orig
unsafe extern fn h5_state_comment(mut hs : *mut h5_state) -> i32
{
    let mut ch : u8;
    let mut idx : *const u8;
    let mut pos : usize;
    let mut offset : usize;
    let mut end : *const u8 = (*hs).s.offset((*hs).len as (isize));

    //TRACE()
    pos = (*hs).pos;
    loop {  //loop1

        idx = memchr((*hs).s.offset(pos as (isize)) as (*const c_void), CHAR_DASH, (*hs).len.wrapping_sub(pos)) as (*const u8);

        /* did not find anything or has less than 3 chars left */
        if idx == CHAR_NULL as (*mut c_void) as (*const u8) || idx > (*hs).s.offset((*hs).len as (isize)).offset(-3isize) {
            (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
            (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
            (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
            (*hs).token_type = html5_type::TAG_COMMENT;
            return 1;
        }
        offset = 1usize;

        /* skip all nulls */ //loop3'
        while idx.offset(offset as (isize)) < end && (*idx.offset(offset as (isize)) as (i32) == CHAR_NULL) {
            offset = offset.wrapping_add(1usize);
        }
        if idx.offset(offset as (isize)) == end { //block 12
            (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
            (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
            (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
            (*hs).token_type = html5_type::TAG_COMMENT;
            return 1;
        }

        ch = *idx.offset(offset as (isize));
        if ch as (i32) != CHAR_DASH && (ch as (i32) != CHAR_BANG) {
            pos = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
            continue;
        }

        /* need to test */
//#if 0
//        /* skip all nulls */
//        while (idx + offset < end && *(idx + offset) == 0) {
//            offset += 1;
//        }
//        if (idx + offset == end) {
//            hs->state = h5_state_eof;
//            hs->token_start = hs->s + hs->pos;
//            hs->token_len = hs->len - hs->pos;
//            hs->token_type = TAG_COMMENT;
//            return 1;
//        }
//#endif

        offset = offset.wrapping_add(1usize);
        if idx.offset(offset as (isize)) == end {  //block 10
            (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
            (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
            (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
            (*hs).token_type = html5_type::TAG_COMMENT;
            return 1;
        }


        ch = *idx.offset(offset as (isize));
        if ch as (i32) != CHAR_GT { //not block 8
            pos = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
            continue;
        }
        offset = offset.wrapping_add(1usize); //block 8

        /* ends in --> or -!> */
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub((*hs).pos);
        (*hs).pos = ((idx.offset(offset as (isize)) as (isize)).wrapping_sub((*hs).s as (isize)) / size_of::<u8>() as (isize)) as (usize);
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_type = html5_type::TAG_COMMENT;
        return 1;
    }
}

#[allow(dead_code)]
unsafe extern fn h5_state_comment_orig(mut hs : *mut h5_state) -> i32 {
    let mut _currentBlock;
    let mut ch : u8;
    let mut idx : *const u8;
    let mut pos : usize;
    let mut offset : usize = 0; //pmc added
    let mut end : *const u8 = (*hs).s.offset((*hs).len as (isize));
    pos = (*hs).pos;
    'loop1: loop {
        idx = memchr(
                  (*hs).s.offset(pos as (isize)) as (*const c_void),
                  45i32,
                  (*hs).len.wrapping_sub(pos)
              ) as (*const u8);
        if idx == 0i32 as (*mut c_void) as (*const u8) || idx > (*hs).s.offset(
                                                                                    (*hs).len as (isize)
                                                                                ).offset(
                                                                                    -3isize
                                                                                ) {
            _currentBlock = 14;
            break;
        }
        offset = 1usize;
        'loop3: loop {
            if !(idx.offset(offset as (isize)) < end && (*idx.offset(
                                                              offset as (isize)
                                                          ) as (i32) == 0i32)) {
                break;
            }
            offset = offset.wrapping_add(1usize);
        }
        if idx.offset(offset as (isize)) == end {
            _currentBlock = 12;
            break;
        }
        ch = *idx.offset(offset as (isize));
        if ch as (i32) != 45i32 && (ch as (i32) != 33i32) {
            pos = (((idx as (isize)).wrapping_sub(
                        (*hs).s as (isize)
                    ) / size_of::<u8>(
                        ) as (isize)) as (usize)).wrapping_add(
                      1usize
                  );
        } else {
            offset = offset.wrapping_add(1usize);
            if idx.offset(offset as (isize)) == end {
                _currentBlock = 10;
                break;
            }
            ch = *idx.offset(offset as (isize));
            if !(ch as (i32) != 62i32) {
                _currentBlock = 8;
                break;
            }
            pos = (((idx as (isize)).wrapping_sub(
                        (*hs).s as (isize)
                    ) / size_of::<u8>(
                        ) as (isize)) as (usize)).wrapping_add(
                      1usize
                  );
        }
    }
    if _currentBlock == 8 {
        offset = offset.wrapping_add(1usize);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = ((idx.offset(
                          offset as (isize)
                      ) as (isize)).wrapping_sub(
                         (*hs).s as (isize)
                     ) / size_of::<u8>() as (isize)) as (usize);
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_type = html5_type::TAG_COMMENT;
        1i32
    } else if _currentBlock == 10 {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::TAG_COMMENT;
        1i32
    } else if _currentBlock == 12 {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::TAG_COMMENT;
        1i32
    } else {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::TAG_COMMENT;
        1i32
    }
}

unsafe extern fn h5_state_cdata(mut hs : *mut h5_state) -> i32 {
    let mut _currentBlock;
    let mut idx : *const u8;
    let mut pos : usize;
    pos = (*hs).pos;
    'loop1: loop {
        idx = memchr(
                  (*hs).s.offset(pos as (isize)) as (*const c_void),
                  93i32,
                  (*hs).len.wrapping_sub(pos)
              ) as (*const u8);
        if idx == 0i32 as (*mut c_void) as (*const u8) || idx > (*hs).s.offset(
                                                                                    (*hs).len as (isize)
                                                                                ).offset(
                                                                                    -3isize
                                                                                ) {
            _currentBlock = 5;
            break;
        }
        if *idx.offset(1isize) as (i32) == 93i32 && (*idx.offset(
                                                          2isize
                                                      ) as (i32) == 62i32) {
            _currentBlock = 4;
            break;
        }
        pos = (((idx as (isize)).wrapping_sub(
                    (*hs).s as (isize)
                ) / size_of::<u8>(
                    ) as (isize)) as (usize)).wrapping_add(
                  1usize
              );
    }
    if _currentBlock == 4 {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
                          (*hs).s as (isize)
                      ) / size_of::<u8>(
                          ) as (isize)) as (usize)).wrapping_add(
                        3usize
                    );
        (*hs).token_type = html5_type::DATA_TEXT;
        1i32
    } else {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::DATA_TEXT;
        1i32
    }
}

unsafe extern fn h5_state_doctype(mut hs : *mut h5_state) -> i32 {
    let mut idx : *const u8;
    (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
    (*hs).token_type = html5_type::DOCTYPE;
    idx = memchr(
              (*hs).s.offset(
                  (*hs).pos as (isize)
              ) as (*const c_void),
              62i32,
              (*hs).len.wrapping_sub((*hs).pos)
          ) as (*const u8);
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
    } else {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
                          (*hs).s as (isize)
                      ) / size_of::<u8>(
                          ) as (isize)) as (usize)).wrapping_add(
                        1usize
                    );
    }
    1i32
}

fn h5_type_to_string(t: html5_type) -> String {
    let s = match t {
        html5_type::DATA_TEXT => "DATA_TEXT",
        html5_type::TAG_NAME_OPEN => "TAG_NAME_OPEN",
        html5_type::TAG_NAME_CLOSE => "TAG_NAME_CLOSE",
        html5_type::TAG_NAME_SELFCLOSE => "TAG_NAME_SELFCLOSE",
        html5_type::TAG_DATA => "TAG_DATA",
        html5_type::TAG_CLOSE => "TAG_CLOSE",
        html5_type::ATTR_NAME => "ATTR_NAME",
        html5_type::ATTR_VALUE => "ATTR_VALUE",
        html5_type::TAG_COMMENT => "TAG_COMMENT",
        html5_type::DOCTYPE => "DOCTYPE"
    };
    s.to_string()
}

fn print_html5_token(hs: *const h5_state) {
    let hs: &h5_state = unsafe { hs.as_ref() }.expect("Couldn't unwrap h5_state");
    let offset = hs.token_start.wrapping_sub(hs.s as usize) as isize;
    let segment = unsafe { ::std::slice::from_raw_parts(hs.s.offset(offset), hs.token_len) };
    let segment = String::from_utf8_lossy(segment);
    let type_ = h5_type_to_string(hs.token_type);
    println!("{}, {}, {:?}", type_, hs.token_len, segment);
}

mod tests {
    use super::*;
    use std::ptr;
    use std::borrow::BorrowMut;

    #[test]
    fn test_html_parse() {
            let mut hs = h5_state {
                s: ptr::null(),
                len: 0usize,
                pos: 0usize,
                is_close: 0,
                state: h5_state_eof,
                token_start: ptr::null(),
                token_len: 0usize,
                token_type: html5_type::TAG_COMMENT,
            };

            let test_html = "<script>alert(document.domain)</script>";
            let hs_ptr = hs.borrow_mut() as *mut h5_state;


            libinjection_h5_init(hs_ptr,  test_html.as_ptr() as *const u8, test_html.len(), html5_flags::DATA_STATE);
            while libinjection_h5_next(hs_ptr) == 1 {
                print_html5_token(hs_ptr);
            }

        }


}

/*

            test string: "<script>alert(document.domain)</script>"
C libinjection html parser               | 100% Rust transpile port
   (90% of libinj XSS parser)            |
---------------------------------------------------------------------------------
TAG_NAME_OPEN, 6, "script"               |  TAG_NAME_OPEN, 6, "script"
TAG_NAME_CLOSE, 1 , ">"                  |  TAG_NAME_CLOSE, 1, ">"
DATA_TEXT, 22 , "alert(document.domain)" |  DATA_TEXT, 22, "alert(document.domain)"
TAG_CLOSE, 6, "script"                   |  TAG_CLOSE, 6, "script"
*/