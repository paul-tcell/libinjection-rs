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
const CHAR_DOUBLE: i32 = 34;
const CHAR_PERCENT: i32 = 37;
const CHAR_SINGLE: i32 = 39;
const CHAR_DASH: i32 = 45;
const CHAR_SLASH: i32 = 47;
const CHAR_LT: i32 = 60;
const CHAR_EQUALS: i32 = 61;
const CHAR_GT: i32 = 62;
const CHAR_QUESTION: i32 = 63;
const CHAR_RIGHTB: i32 = 93;
const CHAR_TICK: i32 = 96;


#[derive(Clone, Copy, PartialEq, Debug)]
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
pub struct h5_state {
    pub s: *const u8,
    pub len: usize,
    pub pos: usize,
    pub is_close: i32,
    pub state: fn(&mut h5_state) -> i32,
    pub token_start: *const u8,
    pub token_len: usize,
    pub token_type: html5_type,
}

#[derive(Clone, Copy)]
pub struct h5_state_safe<'a> {
    pub s: &'a [u8],
    pub len: usize,
    pub pos: usize,
    pub is_close: i32,
    pub state: fn(&mut h5_state_safe) -> i32,
    pub token_start: usize,
    pub token_len: usize,
    pub token_type: html5_type,
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

pub fn libinjection_h5_init_safe<'a>(s: &'a [u8], flags: html5_flags) -> h5_state_safe {
    let state: fn(&mut h5_state_safe) -> i32;
    if flags as (i32) == html5_flags::VALUE_BACK_QUOTE as (i32) {
        state = h5_state_attribute_value_back_quote_safe;
    } else if flags as (i32) == html5_flags::VALUE_DOUBLE_QUOTE as (i32) {
        state = h5_state_attribute_value_double_quote_safe;
    } else if flags as (i32) == html5_flags::VALUE_SINGLE_QUOTE as (i32) {
        state = h5_state_attribute_value_single_quote_safe;
    } else if flags as (i32) == html5_flags::VALUE_NO_QUOTE as (i32) {
        state = h5_state_before_attribute_name_safe;
    } else if flags as (i32) == html5_flags::DATA_STATE as (i32) {
        state = h5_state_data_safe;
    } else {
        state = h5_state_eof_safe;
    }

    h5_state_safe {
        s: s,
        len: s.len(),
        pos: 0,
        is_close: 0,
        state: state,
        token_start: 0,
        token_len: 0,
        token_type: html5_type::DATA_TEXT,
    }
}

#[no_mangle]
pub fn libinjection_h5_init(mut hs: *mut h5_state, mut s: *const u8, mut len: usize, mut flags: html5_flags) {
    unsafe { memset(hs as (*mut c_void), 0i32, size_of::<h5_state>()) };
    let hs = unsafe { hs.as_mut() }.expect("invalid pointer for h5_state.");
    hs.s = s;
    hs.len = len;
    if flags as (i32) == html5_flags::VALUE_BACK_QUOTE as (i32) {
        hs.state = h5_state_attribute_value_back_quote;
    } else if flags as (i32) == html5_flags::VALUE_DOUBLE_QUOTE as (i32) {
        hs.state = h5_state_attribute_value_double_quote;
    } else if flags as (i32) == html5_flags::VALUE_SINGLE_QUOTE as (i32) {
        hs.state = h5_state_attribute_value_single_quote;
    } else if flags as (i32) == html5_flags::VALUE_NO_QUOTE as (i32) {
        hs.state = h5_state_before_attribute_name;
    } else if flags as (i32) == html5_flags::DATA_STATE as (i32) {
        hs.state = h5_state_data;
    }
}

pub fn libinjection_h5_next_safe(hs: &mut h5_state_safe) -> i32 {
    (hs.state)(hs)
}

#[no_mangle]
pub fn libinjection_h5_next(hs: &mut h5_state) -> i32 {
    if hs.state as (*mut c_void) != 0i32 as (*mut c_void) {
        0i32;
    } else {
        panic!("{} in {}:{} function: {}", "hs->state != NULL", file!(), line!(), "libinjection_h5_next");
    }
    (hs.state)(hs)
}

pub fn h5_state_eof(hs: &mut h5_state) -> i32 {
    0i32
}

pub fn h5_state_eof_safe(hs: &mut h5_state_safe) -> i32 {
    0i32
}

fn h5_state_data_safe(hs: &mut h5_state_safe) -> i32 {
    if hs.len < hs.pos {
        panic!("{} in {}:{} function: {}", "hs->len < hs->pos", file!(), line!(), "h5_state_data");
    } else if hs.len == hs.pos {
        hs.token_start = hs.pos;
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::DATA_TEXT;
        hs.state = h5_state_eof_safe;
        if hs.token_len == 0usize {
            return 0i32;
        } else {
            return 1i32;
        }
    }


//             1         2        3
//   01234567890123456789012345678012
//   alert(documentXdomain)</script>
//  "<script>alert(documentXdomain)</script>\000"
    let sub_s: &[u8] = &(hs.s)[hs.pos..hs.len];
    match sub_s.iter().position(|&b| b == CHAR_LT as u8) {
        None => {
            hs.token_start = hs.pos;
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.token_type = html5_type::DATA_TEXT;
            hs.state = h5_state_eof_safe;
            if hs.token_len == 0usize {
                return 0i32;
            }
        }
        Some(idx) => {
            let abs_index = idx + hs.pos;
            hs.token_start = hs.pos;
            hs.token_type = html5_type::DATA_TEXT;
            hs.token_len = idx;
            hs.pos = abs_index + 1;
            hs.state = h5_state_tag_open_safe;
            if hs.token_len == 0usize {
                return h5_state_tag_open_safe(hs);
            }
        }
    }

    1i32
}

fn h5_state_data(hs: &mut h5_state) -> i32 {
    let mut idx: *const u8;
    if hs.len >= hs.pos {
        0i32;
    } else {
        panic!("{} in {}:{} function: {}", "hs->len >= hs->pos", file!(), line!(), "h5_state_data");
    }
    idx = unsafe { memchr(hs.s.offset(hs.pos as (isize)) as (*const c_void), CHAR_LT, hs.len.wrapping_sub(hs.pos)) as (*const u8) };
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::DATA_TEXT;
        hs.state = h5_state_eof;
        if hs.token_len == 0usize {
            return 0i32;
        }
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_type = html5_type::DATA_TEXT;
        hs.token_len = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(hs.pos);
        hs.pos = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
        hs.state = h5_state_tag_open;
        if hs.token_len == 0usize {
            return h5_state_tag_open(hs);
        }
    }
    1i32
}

fn h5_state_tag_open_safe(hs: &mut h5_state_safe) -> i32 {
    let mut ch: u8;
    if hs.pos >= hs.len {
        return 0i32;
    }
    ch = hs.s[hs.pos];
    if ch as (i32) == CHAR_BANG {
        hs.pos = hs.pos.wrapping_add(1usize);
        return h5_state_markup_declaration_open_safe(hs);
    } else if ch as (i32) == CHAR_SLASH {
        hs.pos = hs.pos.wrapping_add(1usize);
        hs.is_close = 1i32;
        return h5_state_end_tag_open_safe(hs);
    } else if ch as (i32) == CHAR_QUESTION {
        hs.pos = hs.pos.wrapping_add(1usize);
        return h5_state_bogus_comment_safe(hs);
    } else if ch as (i32) == CHAR_PERCENT {
        hs.pos = hs.pos.wrapping_add(1usize);
        return h5_state_bogus_comment2_safe(hs);
    } else if ch as (i32) >= b'a' as (i32) && (ch as (i32) <= b'z' as (i32)) ||
        ch as (i32) >= b'A' as (i32) && (ch as (i32) <= b'Z' as (i32)) {
        return h5_state_tag_name_safe(hs);
    } else if ch as (i32) == CHAR_NULL {
        return h5_state_tag_name_safe(hs);
    } else {
        if hs.pos == 0usize {
            return h5_state_data_safe(hs);
        }
        hs.token_start = hs.pos.wrapping_sub(1);
        hs.token_len = 1usize;
        hs.token_type = html5_type::DATA_TEXT;
        hs.state = h5_state_data_safe;
        return 1i32;
    }
}


fn h5_state_tag_open(hs: &mut h5_state) -> i32 {
    let mut ch: u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = unsafe { *hs.s.offset(hs.pos as (isize)) };
        if ch as (i32) == 33i32 {
            hs.pos = hs.pos.wrapping_add(1usize);
            h5_state_markup_declaration_open(hs)
        } else if ch as (i32) == CHAR_SLASH {
            hs.pos = hs.pos.wrapping_add(1usize);
            hs.is_close = 1i32;
            h5_state_end_tag_open(hs)
        } else if ch as (i32) == 63i32 {
            hs.pos = hs.pos.wrapping_add(1usize);
            h5_state_bogus_comment(hs)
        } else if ch as (i32) == CHAR_PERCENT {
            hs.pos = hs.pos.wrapping_add(1usize);
            h5_state_bogus_comment2(hs)
        } else if ch as (i32) >= b'a' as (i32) && (ch as (i32) <= b'z' as (i32)) ||
            ch as (i32) >= b'A' as (i32) && (ch as (i32) <= b'Z' as (i32)) {
            h5_state_tag_name(hs)
        } else if ch as (i32) == 0i32 {
            h5_state_tag_name(hs)
        } else if hs.pos == 0usize {
            h5_state_data(hs)
        } else {
            hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)).offset(-1isize) };
            hs.token_len = 1usize;
            hs.token_type = html5_type::DATA_TEXT;
            hs.state = h5_state_data;
            1i32
        }
    }
}

fn h5_state_end_tag_open_safe(hs: &mut h5_state_safe) -> i32 {
    let mut ch: u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = hs.s[hs.pos];
        if ch as (i32) == CHAR_GT {
            h5_state_data_safe(hs)
        } else if ch as (i32) >= b'a' as (i32) && (ch as (i32) <= b'z' as (i32)) || ch as (i32) >= b'A' as (i32) && (ch as (i32) <= b'Z' as (i32)) {
            h5_state_tag_name_safe(hs)
        } else {
            hs.is_close = 0i32;
            h5_state_bogus_comment_safe(hs)
        }
    }
}

fn h5_state_end_tag_open(hs: &mut h5_state) -> i32 {
    let mut ch: u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = unsafe { *hs.s.offset(hs.pos as (isize)) };
        (if ch as (i32) == 62i32 {
            h5_state_data(hs)
        } else if ch as (i32) >= b'a' as (i32) && (ch as (i32) <= b'z' as (i32)) || ch as (i32) >= b'A' as (i32) && (ch as (i32) <= b'Z' as (i32)) {
            h5_state_tag_name(hs)
        } else {
            hs.is_close = 0i32;
            h5_state_bogus_comment(hs)
        })
    }
}

fn h5_is_white(mut ch: u8) -> i32 {
    unsafe {
        (strchr((*b" \t\n\x0B\x0C\r\0").as_ptr() as *const i8,   //pmc added as i8
                ch as (i32)) != 0i32 as (*mut c_void) as (*mut i8)) as (i32)
    }
}

fn h5_state_tag_name_close_safe(hs: &mut h5_state_safe) -> i32 {
    hs.is_close = 0i32;
    hs.token_start = hs.pos;
    hs.token_len = 1usize;
    hs.token_type = html5_type::TAG_NAME_CLOSE;
    hs.pos = hs.pos.wrapping_add(1usize);
    if hs.pos < hs.len {
        hs.state = h5_state_data_safe;
    } else {
        hs.state = h5_state_eof_safe;
    }
    1i32
}

fn h5_state_tag_name_close(hs: &mut h5_state) -> i32 {
    hs.is_close = 0i32;
    hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
    hs.token_len = 1usize;
    hs.token_type = html5_type::TAG_NAME_CLOSE;
    hs.pos = hs.pos.wrapping_add(1usize);
    if hs.pos < hs.len {
        hs.state = h5_state_data;
    } else {
        hs.state = h5_state_eof;
    }
    1i32
}

fn h5_state_tag_name_safe(hs: &mut h5_state_safe) -> i32 {
    let mut _currentBlock;
    let mut ch: u8;
    let mut pos: usize;
    pos = hs.pos;
    'loop1: loop {
        if !(pos < hs.len) {
            _currentBlock = 2;
            break;
        }
        ch = hs.s[pos];
        if ch as (i32) == 0i32 {
            pos = pos.wrapping_add(1usize);
        } else {
            if h5_is_white(ch) != 0 {
                _currentBlock = 13;
                break;
            }
            if ch as (i32) == CHAR_SLASH {
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
        hs.token_start = hs.pos;
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::TAG_NAME_OPEN;
        hs.state = h5_state_eof_safe;
        1i32
    } else if _currentBlock == 8 {
        hs.token_start = hs.pos;
        hs.token_len = pos.wrapping_sub(hs.pos);
        if hs.is_close != 0 {
            hs.pos = pos.wrapping_add(1usize);
            hs.is_close = 0i32;
            hs.token_type = html5_type::TAG_CLOSE;
            hs.state = h5_state_data_safe;
        } else {
            hs.pos = pos;
            hs.token_type = html5_type::TAG_NAME_OPEN;
            hs.state = h5_state_tag_name_close_safe;
        }
        1i32
    } else if _currentBlock == 12 {
        hs.token_start = hs.pos;
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::TAG_NAME_OPEN;
        hs.pos = pos.wrapping_add(1usize);
        hs.state = h5_state_self_closing_start_tag_safe;
        1i32
    } else {
        hs.token_start = hs.pos;
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::TAG_NAME_OPEN;
        hs.pos = pos.wrapping_add(1usize);
        hs.state = h5_state_before_attribute_name_safe;
        1i32
    }
}

fn h5_state_tag_name(hs: &mut h5_state) -> i32 {
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
            if ch as (i32) == CHAR_SLASH {
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
        hs.state = h5_state_eof;
        1i32
    } else if _currentBlock == 8 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        if hs.is_close != 0 {
            hs.pos = pos.wrapping_add(1usize);
            hs.is_close = 0i32;
            hs.token_type = html5_type::TAG_CLOSE;
            hs.state = h5_state_data;
        } else {
            hs.pos = pos;
            hs.token_type = html5_type::TAG_NAME_OPEN;
            hs.state = h5_state_tag_name_close;
        }
        1i32
    } else if _currentBlock == 12 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::TAG_NAME_OPEN;
        hs.pos = pos.wrapping_add(1usize);
        hs.state = h5_state_self_closing_start_tag;
        1i32
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::TAG_NAME_OPEN;
        hs.pos = pos.wrapping_add(1usize);
        hs.state = h5_state_before_attribute_name;
        1i32
    }
}

fn h5_skip_white_safe(hs: &mut h5_state_safe) -> i32 {
    while hs.pos < hs.len {
        let ch = hs.s[hs.pos];
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

#[allow(unused_parens)]
fn h5_skip_white(hs: &mut h5_state) -> i32 {
    while (hs.pos < hs.len) {
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

fn h5_state_before_attribute_name_safe(hs: &mut h5_state_safe) -> i32 {
    let mut ch: i32;
    ch = h5_skip_white_safe(hs);
    if ch == 62i32 {
        hs.state = h5_state_data_safe;
        hs.token_start = hs.pos;
        hs.token_len = 1usize;
        hs.token_type = html5_type::TAG_NAME_CLOSE;
        hs.pos = hs.pos.wrapping_add(1usize);
        1i32
    } else if ch == CHAR_SLASH {
        hs.pos = hs.pos.wrapping_add(1usize);
        h5_state_self_closing_start_tag_safe(hs)
    } else if ch == -1i32 {
        0i32
    } else {
        h5_state_attribute_name_safe(hs)
    }
}

fn h5_state_before_attribute_name(hs: &mut h5_state) -> i32 {
    let mut ch: i32;
    ch = h5_skip_white(hs);
    if ch == 62i32 {
        hs.state = h5_state_data;
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = 1usize;
        hs.token_type = html5_type::TAG_NAME_CLOSE;
        hs.pos = hs.pos.wrapping_add(1usize);
        1i32
    } else if ch == CHAR_SLASH {
        hs.pos = hs.pos.wrapping_add(1usize);
        h5_state_self_closing_start_tag(hs)
    } else if ch == -1i32 {
        0i32
    } else {
        h5_state_attribute_name(hs)
    }
}

fn h5_state_attribute_name_safe(hs: &mut h5_state_safe) -> i32 {
    let mut ch: u8;
    let mut pos: usize;
    pos = hs.pos.wrapping_add(1usize);
    'loop1: loop {
        if !(pos < hs.len) {
            hs.token_start = hs.pos;
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.token_type = html5_type::ATTR_NAME;
            hs.state = h5_state_eof_safe;
            hs.pos = hs.len;
            return 1i32;
        }
        ch = hs.s[pos];
        if h5_is_white(ch) != 0 {
            hs.token_start = hs.pos;
            hs.token_len = pos.wrapping_sub(hs.pos);
            hs.token_type = html5_type::ATTR_NAME;
            hs.state = h5_state_after_attribute_name_safe;
            hs.pos = pos.wrapping_add(1usize);
            return 1i32;
        }
        if ch as (i32) == CHAR_SLASH {
            hs.token_start = hs.pos;
            hs.token_len = pos.wrapping_sub(hs.pos);
            hs.token_type = html5_type::ATTR_NAME;
            hs.state = h5_state_self_closing_start_tag_safe;
            hs.pos = pos.wrapping_add(1usize);
            return 1i32;
        }
        if ch as (i32) == CHAR_EQUALS {
            hs.token_start = hs.pos;
            hs.token_len = pos.wrapping_sub(hs.pos);
            hs.token_type = html5_type::ATTR_NAME;
            hs.state = h5_state_before_attribute_value_safe;
            hs.pos = pos.wrapping_add(1usize);
            return 1i32;
        }
        if ch as (i32) == 62i32 {
            hs.token_start = hs.pos;
            hs.token_len = pos.wrapping_sub(hs.pos);
            hs.token_type = html5_type::ATTR_NAME;
            hs.state = h5_state_tag_name_close_safe;
            hs.pos = pos;
            return 1i32;
        }
        pos = pos.wrapping_add(1usize);
    }
}

fn h5_state_attribute_name(hs: &mut h5_state) -> i32 {
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
        if ch as (i32) == CHAR_SLASH {
            _currentBlock = 10;
            break;
        }
        if ch as (i32) == CHAR_EQUALS {
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
        hs.state = h5_state_eof;
        hs.pos = hs.len;
        1i32
    } else if _currentBlock == 8 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_NAME;
        hs.state = h5_state_tag_name_close;
        hs.pos = pos;
        1i32
    } else if _currentBlock == 9 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_NAME;
        hs.state = h5_state_before_attribute_value;
        hs.pos = pos.wrapping_add(1usize);
        1i32
    } else if _currentBlock == 10 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_NAME;
        hs.state = h5_state_self_closing_start_tag;
        hs.pos = pos.wrapping_add(1usize);
        1i32
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_NAME;
        hs.state = h5_state_after_attribute_name;
        hs.pos = pos.wrapping_add(1usize);
        1i32
    }
}

fn h5_state_after_attribute_name_safe(hs: &mut h5_state_safe) -> i32 {
    let mut c: i32;
    c = h5_skip_white_safe(hs);
    if c == 62i32 {
        h5_state_tag_name_close_safe(hs)
    } else if c == CHAR_EQUALS {
        hs.pos = hs.pos.wrapping_add(1usize);
        h5_state_before_attribute_value_safe(hs)
    } else if c == CHAR_SLASH {
        hs.pos = hs.pos.wrapping_add(1usize);
        h5_state_self_closing_start_tag_safe(hs)
    } else if c == -1i32 {
        0i32
    } else {
        h5_state_attribute_name_safe(hs)
    }
}

fn h5_state_after_attribute_name(hs: &mut h5_state) -> i32 {
    let mut c: i32;
    c = h5_skip_white(hs);
    if c == 62i32 {
        h5_state_tag_name_close(hs)
    } else if c == CHAR_EQUALS {
        hs.pos = hs.pos.wrapping_add(1usize);
        h5_state_before_attribute_value(hs)
    } else if c == CHAR_SLASH {
        hs.pos = hs.pos.wrapping_add(1usize);
        h5_state_self_closing_start_tag(hs)
    } else if c == -1i32 {
        0i32
    } else {
        h5_state_attribute_name(hs)
    }
}

fn h5_state_before_attribute_value_safe(hs: &mut h5_state_safe) -> i32 {
    let mut c: i32;
    c = h5_skip_white_safe(hs);
    if c == -1i32 {
        hs.state = h5_state_eof_safe;
        0i32
    } else if c == CHAR_DOUBLE {
        h5_state_attribute_value_double_quote_safe(hs)
    } else if c == CHAR_SINGLE {
        h5_state_attribute_value_single_quote_safe(hs)
    } else if c == CHAR_TICK {
        h5_state_attribute_value_back_quote_safe(hs)
    } else {
        h5_state_attribute_value_no_quote_safe(hs)
    }
}

fn h5_state_before_attribute_value(hs: &mut h5_state) -> i32 {
    let mut c: i32;
    c = h5_skip_white(hs);
    if c == -1i32 {
        hs.state = h5_state_eof;
        0i32
    } else if c == CHAR_DOUBLE {
        h5_state_attribute_value_double_quote(hs)
    } else if c == CHAR_SINGLE {
        h5_state_attribute_value_single_quote(hs)
    } else if c == CHAR_TICK {
        h5_state_attribute_value_back_quote(hs)
    } else {
        h5_state_attribute_value_no_quote(hs)
    }
}

fn h5_state_attribute_value_quote_safe(hs: &mut h5_state_safe, mut qchar: u8) -> i32 {
    if hs.pos > 0 {
        hs.pos += 1;
    }
    let sub_s: &[u8] = &(hs.s)[hs.pos..hs.len];
    match sub_s.iter().position(|&b| b == qchar as u8) {
        None => {
            hs.token_start = hs.pos;
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.token_type = html5_type::ATTR_VALUE;
            hs.state = h5_state_eof_safe;
        }
        Some(idx) => {
            //idx is the relative offset, need abs
            hs.token_start = hs.pos;
            hs.token_len = idx; //(idx - hs->s) - hs->pos + hs->pos
            hs.token_type = html5_type::ATTR_VALUE;
            hs.state = h5_state_after_attribute_value_quoted_state_safe;
            hs.pos += hs.token_len + 1;
        }
    }
    1i32
}

fn h5_state_attribute_value_quote(hs: &mut h5_state, mut qchar: u8) -> i32 {
    let mut idx: *const u8;
    if hs.pos > 0usize {
        hs.pos = hs.pos.wrapping_add(1usize);
    }
    idx = unsafe { memchr(hs.s.offset(hs.pos as (isize)) as (*const c_void), qchar as (i32), hs.len.wrapping_sub(hs.pos)) as (*const u8) };
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_VALUE;
        hs.state = h5_state_eof;
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_VALUE;
        hs.state = h5_state_after_attribute_value_quoted_state;
        hs.pos = hs.pos.wrapping_add(hs.token_len.wrapping_add(1usize));
    }
    1i32
}

fn h5_state_attribute_value_double_quote_safe(hs: &mut h5_state_safe) -> i32 {
    h5_state_attribute_value_quote_safe(hs, 34u8)
}

fn h5_state_attribute_value_single_quote_safe(hs: &mut h5_state_safe) -> i32 {
    h5_state_attribute_value_quote_safe(hs, 39u8)
}

fn h5_state_attribute_value_back_quote_safe(hs: &mut h5_state_safe) -> i32 {
    h5_state_attribute_value_quote_safe(hs, 96u8)
}

fn h5_state_attribute_value_double_quote(hs: &mut h5_state) -> i32 {
    h5_state_attribute_value_quote(hs, 34u8)
}

fn h5_state_attribute_value_single_quote(hs: &mut h5_state) -> i32 {
    h5_state_attribute_value_quote(hs, 39u8)
}

fn h5_state_attribute_value_back_quote(hs: &mut h5_state) -> i32 {
    h5_state_attribute_value_quote(hs, 96u8)
}


fn h5_state_attribute_value_no_quote_safe(hs: &mut h5_state_safe) -> i32 {
    let mut _currentBlock;
    let mut ch: u8;
    let mut pos: usize;
    pos = hs.pos;
    'loop1: loop {
        if !(pos < hs.len) {
            _currentBlock = 2;
            break;
        }
        ch = hs.s[pos];
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
        hs.state = h5_state_eof_safe;
        hs.token_start = hs.pos;
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_VALUE;
        1i32
    } else if _currentBlock == 6 {
        hs.token_type = html5_type::ATTR_VALUE;
        hs.token_start = hs.pos;
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.pos = pos;
        hs.state = h5_state_tag_name_close_safe;
        1i32
    } else {
        hs.token_type = html5_type::ATTR_VALUE;
        hs.token_start = hs.pos;
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.pos = pos.wrapping_add(1usize);
        hs.state = h5_state_before_attribute_name_safe;
        1i32
    }
}

fn h5_state_attribute_value_no_quote(hs: &mut h5_state) -> i32 {
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
        hs.state = h5_state_eof;
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::ATTR_VALUE;
        1i32
    } else if _currentBlock == 6 {
        hs.token_type = html5_type::ATTR_VALUE;
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.pos = pos;
        hs.state = h5_state_tag_name_close;
        1i32
    } else {
        hs.token_type = html5_type::ATTR_VALUE;
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = pos.wrapping_sub(hs.pos);
        hs.pos = pos.wrapping_add(1usize);
        hs.state = h5_state_before_attribute_name;
        1i32
    }
}

fn h5_state_after_attribute_value_quoted_state_safe(hs: &mut h5_state_safe) -> i32 {
    let mut ch: u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = hs.s[hs.pos];
        (if h5_is_white(ch) != 0 {
            hs.pos = hs.pos.wrapping_add(1usize);
            h5_state_before_attribute_name_safe(hs)
        } else if ch as (i32) == CHAR_SLASH {
            hs.pos = hs.pos.wrapping_add(1usize);
            h5_state_self_closing_start_tag_safe(hs)
        } else if ch as (i32) == 62i32 {
            hs.token_start = hs.pos;
            hs.token_len = 1usize;
            hs.token_type = html5_type::TAG_NAME_CLOSE;
            hs.pos = hs.pos.wrapping_add(1usize);
            hs.state = h5_state_data_safe;
            1i32
        } else {
            h5_state_before_attribute_name_safe(hs)
        })
    }
}

fn h5_state_after_attribute_value_quoted_state(hs: &mut h5_state) -> i32 {
    let mut ch: u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = unsafe { *hs.s.offset(hs.pos as (isize)) };
        (if h5_is_white(ch) != 0 {
            hs.pos = hs.pos.wrapping_add(1usize);
            h5_state_before_attribute_name(hs)
        } else if ch as (i32) == CHAR_SLASH {
            hs.pos = hs.pos.wrapping_add(1usize);
            h5_state_self_closing_start_tag(hs)
        } else if ch as (i32) == 62i32 {
            hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
            hs.token_len = 1usize;
            hs.token_type = html5_type::TAG_NAME_CLOSE;
            hs.pos = hs.pos.wrapping_add(1usize);
            hs.state = h5_state_data;
            1i32
        } else {
            h5_state_before_attribute_name(hs)
        })
    }
}

fn h5_state_self_closing_start_tag_safe(hs: &mut h5_state_safe) -> i32 {
    let mut ch: u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = hs.s[hs.pos];
        (if ch as (i32) == 62i32 {
            if hs.pos > 0usize {
                0i32;
            } else {
                panic!("{} in {}:{} function: {}", "hs->pos > 0", file!(), line!(), "h5_state_self_closing_start_tag");
            }
            hs.token_start = hs.pos - 1;
            hs.token_len = 2usize;
            hs.token_type = html5_type::TAG_NAME_SELFCLOSE;
            hs.state = h5_state_data_safe;
            hs.pos = hs.pos.wrapping_add(1usize);
            1i32
        } else {
            h5_state_before_attribute_name_safe(hs)
        })
    }
}

fn h5_state_self_closing_start_tag(hs: &mut h5_state) -> i32 {
    let mut ch: u8;
    if hs.pos >= hs.len {
        0i32
    } else {
        ch = unsafe { *hs.s.offset(hs.pos as (isize)) };
        (if ch as (i32) == 62i32 {
            if hs.pos > 0usize {
                0i32;
            } else {
                panic!("{} in {}:{} function: {}", "hs->pos > 0", file!(), line!(), "h5_state_self_closing_start_tag");
            }
            hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)).offset(-1isize) };
            hs.token_len = 2usize;
            hs.token_type = html5_type::TAG_NAME_SELFCLOSE;
            hs.state = h5_state_data;
            hs.pos = hs.pos.wrapping_add(1usize);
            1i32
        } else {
            h5_state_before_attribute_name(hs)
        })
    }
}

fn h5_state_bogus_comment_safe(hs: &mut h5_state_safe) -> i32 {
    let sub_s: &[u8] = &(hs.s)[hs.pos..hs.len];
    match sub_s.iter().position(|&b| b == CHAR_GT as u8) {
        None => {
            hs.token_start = hs.pos;
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.pos = hs.len;
            hs.state = h5_state_eof_safe;
        }
        Some(idx) => {  //idx + hs.pos is the absolute index
            hs.token_start = hs.pos;
            hs.token_len = idx;
            hs.pos = idx + hs.pos + 1;
            hs.state = h5_state_data_safe;
        }
    }
    hs.token_type = html5_type::TAG_COMMENT;
    1i32
}

fn h5_state_bogus_comment(hs: &mut h5_state) -> i32 {
    let mut idx: *const u8;
    idx = unsafe { memchr(hs.s.offset(hs.pos as (isize)) as (*const c_void), 62i32, hs.len.wrapping_sub(hs.pos)) as (*const u8) };
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.pos = hs.len;
        hs.state = h5_state_eof;
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(hs.pos);
        hs.pos = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
        hs.state = h5_state_data;
    }
    hs.token_type = html5_type::TAG_COMMENT;
    1i32
}


fn h5_state_bogus_comment2_safe(hs: &mut h5_state_safe) -> i32 {
    let mut pos: usize;
    pos = hs.pos;
    'loop1: loop {
        let sub_s: &[u8] = &(hs.s)[pos..hs.len];
        let idx = sub_s.iter().position(|&b| b == CHAR_PERCENT as u8);
        //(idx.unwrap() + hs.pos) is the absolute index
        if idx.is_none() || (idx.unwrap() + hs.pos) + 1 >= hs.len {
            hs.token_start = hs.pos;
            hs.token_len = hs.len - hs.pos;
            hs.pos = hs.len;
            hs.token_type = html5_type::TAG_COMMENT;
            hs.state = h5_state_eof_safe;
            return 1;
        }
        let abs_idx = idx.unwrap() + hs.pos;

        if hs.s[abs_idx + 1] as i32 != CHAR_GT {
            pos = abs_idx + 1;
            continue;
        }
        /* ends in %> */
        hs.token_start = hs.pos;
        hs.token_len = abs_idx - hs.pos;
        hs.pos = abs_idx + 2;
        hs.state = h5_state_data_safe;
        hs.token_type = html5_type::TAG_COMMENT;
        return 1;
    }
}

fn h5_state_bogus_comment2(hs: &mut h5_state) -> i32 {
    let mut _currentBlock;
    let mut idx: *const u8;
    let mut pos: usize;
    pos = hs.pos;
    'loop1: loop {
        idx = unsafe { memchr(hs.s.offset(pos as (isize)) as (*const c_void), CHAR_PERCENT, hs.len.wrapping_sub(pos)) as (*const u8) };
        if idx == 0i32 as (*mut c_void) as (*const u8) || unsafe { idx.offset(1isize) } >= unsafe { hs.s.offset(hs.len as (isize)) } {
            _currentBlock = 5;
            break;
        }
        if !(unsafe { *idx.offset(1isize) } as (i32) != 62i32) {
            _currentBlock = 3;
            break;
        }
        pos = (((idx as (isize)).wrapping_sub(
            hs.s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(
            1usize
        );
    }
    if _currentBlock == 3 {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = (((idx as (isize)).wrapping_sub(
            hs.s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(
            hs.pos
        );
        hs.pos = (((idx as (isize)).wrapping_sub(
            hs.s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(
            2usize
        );
        hs.state = h5_state_data;
        hs.token_type = html5_type::TAG_COMMENT;
        1i32
    } else {
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.pos = hs.len;
        hs.token_type = html5_type::TAG_COMMENT;
        hs.state = h5_state_eof;
        1i32
    }
}

fn h5_state_markup_declaration_open_safe(hs: &mut h5_state_safe) -> i32 {
    let mut remaining: usize;
    remaining = hs.len.wrapping_sub(hs.pos);
    if remaining >= 7usize
        && (hs.s[hs.pos.wrapping_add(0usize)] == b'D' || hs.s[hs.pos.wrapping_add(0usize)] == b'd')
        && (hs.s[hs.pos.wrapping_add(1usize)] == b'O' || hs.s[hs.pos.wrapping_add(1usize)] == b'o')
        && (hs.s[hs.pos.wrapping_add(2usize)] == b'C' || hs.s[hs.pos.wrapping_add(2usize)] == b'c')
        && (hs.s[hs.pos.wrapping_add(3usize)] == b'T' || hs.s[hs.pos.wrapping_add(3usize)] == b't')
        && (hs.s[hs.pos.wrapping_add(4usize)] == b'Y' || hs.s[hs.pos.wrapping_add(4usize)] == b'y')
        && (hs.s[hs.pos.wrapping_add(5usize)] == b'P' || hs.s[hs.pos.wrapping_add(5usize)] == b'p')
        && (hs.s[hs.pos.wrapping_add(6usize)] == b'E' || hs.s[hs.pos.wrapping_add(6usize)] == b'e') {
        return h5_state_doctype_safe(hs);
    } else if remaining >= 7usize
        && hs.s[hs.pos.wrapping_add(0usize)] == b'['
        && hs.s[hs.pos.wrapping_add(1usize)] == b'C'
        && hs.s[hs.pos.wrapping_add(2usize)] == b'D'
        && hs.s[hs.pos.wrapping_add(3usize)] == b'A'
        && hs.s[hs.pos.wrapping_add(4usize)] == b'T'
        && hs.s[hs.pos.wrapping_add(5usize)] == b'A'
        && hs.s[hs.pos.wrapping_add(6usize)] == b'[' {
        hs.pos = hs.pos.wrapping_add(7usize);
        return h5_state_cdata_safe(hs);
    } else if remaining >= 2usize
        && hs.s[hs.pos.wrapping_add(0usize)] == b'-'
        && hs.s[hs.pos.wrapping_add(1usize)] == b'-' {
        hs.pos = hs.pos.wrapping_add(2usize);
        return h5_state_comment_safe(hs);
    }
    return h5_state_bogus_comment_safe(hs);
}


fn h5_state_markup_declaration_open(hs: &mut h5_state) -> i32 {
    let mut remaining: usize;
    remaining = hs.len.wrapping_sub(hs.pos);
    unsafe {
        if remaining >= 7usize
            && (*hs.s.offset(hs.pos.wrapping_add(0usize) as (isize)) as (i32) == b'D' as (i32) || *hs.s.offset(hs.pos.wrapping_add(0usize) as (isize)) as (i32) == b'd' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(1usize) as (isize)) as (i32) == b'O' as (i32) || *hs.s.offset(hs.pos.wrapping_add(1usize) as (isize)) as (i32) == b'o' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(2usize) as (isize)) as (i32) == b'C' as (i32) || *hs.s.offset(hs.pos.wrapping_add(2usize) as (isize)) as (i32) == b'c' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(3usize) as (isize)) as (i32) == b'T' as (i32) || *hs.s.offset(hs.pos.wrapping_add(3usize) as (isize)) as (i32) == b't' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(4usize) as (isize)) as (i32) == b'Y' as (i32) || *hs.s.offset(hs.pos.wrapping_add(4usize) as (isize)) as (i32) == b'y' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(5usize) as (isize)) as (i32) == b'P' as (i32) || *hs.s.offset(hs.pos.wrapping_add(5usize) as (isize)) as (i32) == b'p' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(6usize) as (isize)) as (i32) == b'E' as (i32) || *hs.s.offset(hs.pos.wrapping_add(6usize) as (isize)) as (i32) == b'e' as (i32)) {
            h5_state_doctype(hs)
        } else if remaining >= 7usize
            && (*hs.s.offset(hs.pos.wrapping_add(0usize) as (isize)) as (i32) == b'[' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(1usize) as (isize)) as (i32) == b'C' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(2usize) as (isize)) as (i32) == b'D' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(3usize) as (isize)) as (i32) == b'A' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(4usize) as (isize)) as (i32) == b'T' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(5usize) as (isize)) as (i32) == b'A' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(6usize) as (isize)) as (i32) == b'[' as (i32)) {
            hs.pos = hs.pos.wrapping_add(7usize);
            h5_state_cdata(hs)
        } else if remaining >= 2usize
            && (*hs.s.offset(hs.pos.wrapping_add(0usize) as (isize)) as (i32) == b'-' as (i32))
            && (*hs.s.offset(hs.pos.wrapping_add(1usize) as (isize)) as (i32) == b'-' as (i32)) {
            hs.pos = hs.pos.wrapping_add(2usize);
            h5_state_comment(hs)
        } else {
            h5_state_bogus_comment(hs)
        }
    }
}

//pmc - rewrote - h5_state_comment_see orig
fn h5_state_comment_safe(hs: &mut h5_state_safe) -> i32
{
    let mut ch: u8;
    let mut pos: usize;
    let mut offset: usize;
    let mut end = hs.len;

    //TRACE()
    pos = hs.pos;
    loop {  //loop1

        let sub_s: &[u8] = &(hs.s)[pos..hs.len];
        let mut idxo = sub_s.iter().position(|&b| b == CHAR_DASH as u8);
        //idxo.unwrap() + hs.pos is the absolute index
        /* did not find anything or has less than 3 chars left */
        if idxo.is_none() || (idxo.unwrap() + pos) > hs.len - 3 {
            hs.state = h5_state_eof_safe;
            hs.token_start = hs.pos;
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.token_type = html5_type::TAG_COMMENT;
            return 1;
        }
        let idx = idxo.unwrap() + pos;  //abs index
        offset = 1usize;

        /* skip all nulls */ //loop3'
        while idx + offset < end && hs.s[idx + offset] as i32 == CHAR_NULL {
            offset = offset.wrapping_add(1usize);
        }
        if idx + offset == end { //block 12
            hs.state = h5_state_eof_safe;
            hs.token_start = hs.pos;
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.token_type = html5_type::TAG_COMMENT;
            return 1;
        }

        ch = hs.s[idx + offset];
        if ch as (i32) != CHAR_DASH && (ch as (i32) != CHAR_BANG) {
            pos = idx + 1;
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
        if idx + offset == end {  //block 10
            hs.state = h5_state_eof_safe;
            hs.token_start = hs.pos;
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.token_type = html5_type::TAG_COMMENT;
            return 1;
        }


        ch = hs.s[idx + offset];
        if ch as (i32) != CHAR_GT { //not block 8
            pos = idx + 1;
            continue;
        }
        offset = offset.wrapping_add(1usize); //block 8

        /* ends in --> or -!> */
        hs.token_start = hs.pos;
        hs.token_len = idx - hs.pos;
        hs.pos = idx + offset;
        hs.state = h5_state_data_safe;
        hs.token_type = html5_type::TAG_COMMENT;
        return 1;
    }
}


//pmc - rewrote - h5_state_comment_see orig
fn h5_state_comment(hs: &mut h5_state) -> i32
{
    let mut ch: u8;
    let mut idx: *const u8;
    let mut pos: usize;
    let mut offset: usize;
    let mut end: *const u8 = unsafe { hs.s.offset(hs.len as (isize)) };

    //TRACE()
    pos = hs.pos;
    loop {  //loop1

        idx = unsafe { memchr(hs.s.offset(pos as (isize)) as (*const c_void), CHAR_DASH, hs.len.wrapping_sub(pos)) as (*const u8) };

        /* did not find anything or has less than 3 chars left */
        if idx == CHAR_NULL as (*mut c_void) as (*const u8) || idx > unsafe { hs.s.offset(hs.len as (isize)).offset(-3isize) } {
            hs.state = h5_state_eof;
            hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.token_type = html5_type::TAG_COMMENT;
            return 1;
        }
        offset = 1usize;

        /* skip all nulls */ //loop3'
        while unsafe { idx.offset(offset as (isize)) } < end && (unsafe { *idx.offset(offset as (isize)) } as (i32) == CHAR_NULL) {
            offset = offset.wrapping_add(1usize);
        }
        if unsafe { idx.offset(offset as (isize)) } == end { //block 12
            hs.state = h5_state_eof;
            hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.token_type = html5_type::TAG_COMMENT;
            return 1;
        }

        ch = unsafe { *idx.offset(offset as (isize)) };
        if ch as (i32) != CHAR_DASH && (ch as (i32) != CHAR_BANG) {
            pos = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
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
        if unsafe { idx.offset(offset as (isize)) } == end {  //block 10
            hs.state = h5_state_eof;
            hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
            hs.token_len = hs.len.wrapping_sub(hs.pos);
            hs.token_type = html5_type::TAG_COMMENT;
            return 1;
        }


        ch = unsafe { *idx.offset(offset as (isize)) };
        if ch as (i32) != CHAR_GT { //not block 8
            pos = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
            continue;
        }
        offset = offset.wrapping_add(1usize); //block 8

        /* ends in --> or -!> */
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(hs.pos);
        hs.pos = ((unsafe { idx.offset(offset as (isize)) } as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize);
        hs.state = h5_state_data;
        hs.token_type = html5_type::TAG_COMMENT;
        return 1;
    }
}

#[allow(dead_code)]
unsafe fn h5_state_comment_orig(mut hs: *mut h5_state) -> i32 {
    let mut _currentBlock;
    let mut ch: u8;
    let mut idx: *const u8;
    let mut pos: usize;
    let mut offset: usize = 0; //pmc added
    let mut end: *const u8 = (*hs).s.offset((*hs).len as (isize));
    pos = (*hs).pos;
    'loop1: loop {
        idx = memchr(
            (*hs).s.offset(pos as (isize)) as (*const c_void),
            CHAR_DASH,
            (*hs).len.wrapping_sub(pos),
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
            if !(idx.offset(offset as (isize)) < end && (*idx.offset(offset as (isize)) as (i32) == 0i32)) {
                break;
            }
        }
        if idx.offset(offset as (isize)) == end {
            _currentBlock = 12;
            break;
        }
        ch = *idx.offset(offset as (isize));
        if ch as (i32) != CHAR_DASH && (ch as (i32) != 33i32) {
            pos = (((idx as (isize)).wrapping_sub(
                (*hs).s as (isize)
            ) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(
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
            ) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(
                1usize
            );
        }
    }
    if _currentBlock == 8 {
        offset = offset.wrapping_add(1usize);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
            (*hs).s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(
            (*hs).pos
        );
        (*hs).pos = ((idx.offset(
            offset as (isize)
        ) as (isize)).wrapping_sub(
            (*hs).s as (isize)
        ) / size_of::<u8>() as (isize)) as (usize);
        (*hs).state = h5_state_data;
        (*hs).token_type = html5_type::TAG_COMMENT;
        1i32
    } else if _currentBlock == 10 {
        (*hs).state = h5_state_eof;
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::TAG_COMMENT;
        1i32
    } else if _currentBlock == 12 {
        (*hs).state = h5_state_eof;
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::TAG_COMMENT;
        1i32
    } else {
        (*hs).state = h5_state_eof;
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::TAG_COMMENT;
        1i32
    }
}

fn h5_state_cdata_safe(hs: &mut h5_state_safe) -> i32 {
    let mut pos: usize;
    pos = hs.pos;
    'loop1: loop {
        let sub_s: &[u8] = &(hs.s)[pos..hs.len];
        let idx = sub_s.iter().position(|&b| b == CHAR_RIGHTB as u8);
        match idx {
            None => {
                hs.state = h5_state_eof_safe;
                hs.token_start = hs.pos;
                hs.token_len = hs.len.wrapping_sub(hs.pos);
                hs.token_type = html5_type::DATA_TEXT;
                return 1i32;
            }
            Some(idx) if idx + pos > (hs.len + 3) => {
                hs.state = h5_state_eof_safe;
                hs.token_start = hs.pos;
                hs.token_len = hs.len.wrapping_sub(hs.pos);
                hs.token_type = html5_type::DATA_TEXT;
                return 1i32;
            }
            Some(idx) if sub_s[idx + 1] as i32 == CHAR_RIGHTB && sub_s[idx + 2] as i32 == CHAR_GT => {
                hs.state = h5_state_data_safe;
                hs.token_start = hs.pos;
                hs.token_len = idx + pos - hs.pos;
                hs.pos = idx + pos + 3;
                hs.token_type = html5_type::DATA_TEXT;
                return 1i32;
            }
            Some(idx) => {
                pos = idx + pos + 1;
            }
        };
    }
}

fn h5_state_cdata(hs: &mut h5_state) -> i32 {
    let mut _currentBlock;
    let mut idx: *const u8;
    let mut pos: usize;
    pos = hs.pos;
    'loop1: loop {
        idx = unsafe { memchr(hs.s.offset(pos as (isize)) as (*const c_void), 93i32, hs.len.wrapping_sub(pos)) as (*const u8) };
        if idx == 0i32 as (*mut c_void) as (*const u8) || idx > unsafe { hs.s.offset(hs.len as (isize)).offset(-3isize) } {
            _currentBlock = 5;
            break;
        }
        if unsafe { *idx.offset(1isize) } as (i32) == 93i32 && (unsafe { *idx.offset(2isize) } as (i32) == 62i32) {
            _currentBlock = 4;
            break;
        }
        pos = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
    }
    if _currentBlock == 4 {
        hs.state = h5_state_data;
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(hs.pos);
        hs.pos = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(3usize);
        hs.token_type = html5_type::DATA_TEXT;
        1i32
    } else {
        hs.state = h5_state_eof;
        hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
        hs.token_len = hs.len.wrapping_sub(hs.pos);
        hs.token_type = html5_type::DATA_TEXT;
        1i32
    }
}

fn h5_state_doctype_safe(hs: &mut h5_state_safe) -> i32 {
    hs.token_start = hs.pos;
    hs.token_type = html5_type::DOCTYPE;
    let sub_s: &[u8] = &(hs.s)[hs.pos..hs.len];
    match sub_s.iter().position(|&b| b == CHAR_GT as u8) {
        None => {
            hs.state = h5_state_eof_safe;
            hs.token_len = hs.len.wrapping_sub(hs.pos);
        }
        Some(idx) => { //idx + hs.pos is the absolute index, hs.pos -hs.pos cancel each other out
            hs.state = h5_state_data_safe;
            hs.token_len = idx;
            hs.pos = idx + hs.pos + 1;
        }
    }
    1i32
}

fn h5_state_doctype(hs: &mut h5_state) -> i32 {
    let mut idx: *const u8;
    hs.token_start = unsafe { hs.s.offset(hs.pos as (isize)) };
    hs.token_type = html5_type::DOCTYPE;
    idx = unsafe { memchr(hs.s.offset(hs.pos as (isize)) as (*const c_void), 62i32, hs.len.wrapping_sub(hs.pos)) as (*const u8) };
    if idx == 0i32 as (*mut c_void) as (*const u8) {
        hs.state = h5_state_eof;
        hs.token_len = hs.len.wrapping_sub(hs.pos);
    } else {
        hs.state = h5_state_data;
        hs.token_len = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_sub(hs.pos);
        hs.pos = (((idx as (isize)).wrapping_sub(hs.s as (isize)) / size_of::<u8>() as (isize)) as (usize)).wrapping_add(1usize);
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

fn print_html5_token_safe(hs: &h5_state_safe) {
    let seg = &hs.s[hs.token_start..hs.token_len + hs.token_start];
    let seg = String::from_utf8_lossy(seg);
    let type_ = h5_type_to_string(hs.token_type);

    println!("{}, {}, {:?}", type_, hs.token_len, seg);
}

fn print_html5_token(hs: *const h5_state) {
    let hs: &h5_state = unsafe { hs.as_ref() }.expect("Couldn't unwrap h5_state");
    let offset = unsafe { hs.token_start.offset(-(hs.s as isize)) } as isize;
    let segment = unsafe { ::std::slice::from_raw_parts(hs.s.offset(offset), hs.token_len) };
    let segment = String::from_utf8_lossy(segment);
    let type_ = h5_type_to_string(hs.token_type);
    println!("{}, {}, {:?}", type_, hs.token_len, segment);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    use std::borrow::BorrowMut;

    #[derive(Clone)]
    pub struct Both<'a> {
        hs_safe: h5_state_safe<'a>,
        hs_unsafe: h5_state,
    }

    fn test_init<'a>(s: &'a [u8], flags: html5_flags) -> Both {
        let mut hs_unsafe = h5_state {
            s: ptr::null(),
            len: 0usize,
            pos: 0usize,
            is_close: 0,
            state: h5_state_eof,
            token_start: ptr::null(),
            token_len: 0usize,
            token_type: html5_type::DATA_TEXT,
        };
        let hs_safe = libinjection_h5_init_safe(s, flags);
        let hs_unsafe_ptr = hs_unsafe.borrow_mut() as *mut h5_state;
        libinjection_h5_init(hs_unsafe_ptr, s.as_ptr() as *const u8, s.len(), html5_flags::DATA_STATE);
        Both { hs_safe: hs_safe, hs_unsafe: hs_unsafe }
    }

    fn test_next_token(hs: &mut Both) -> i32 {
        let safe_next = libinjection_h5_next_safe(&mut hs.hs_safe);
        let unsafe_next = libinjection_h5_next(&mut hs.hs_unsafe);
        assert_eq!(safe_next, unsafe_next);
        safe_next
    }

    fn convert(hs: Both) -> TestTriple {
        let hs_safe = hs.hs_safe;
        let safe_tok = TestTriple {
            type_: hs_safe.token_type,
            len: hs_safe.token_len,
            value: &hs_safe.s[hs_safe.token_start..hs_safe.token_len + hs_safe.token_start],
        };
        let unsafe_tok = convertX(hs.hs_unsafe);
        assert_eq!(safe_tok, unsafe_tok);
        safe_tok
    }

    fn convertX<'a>(hs: h5_state) -> TestTriple<'a> {
        let hs: &h5_state = &hs;//unsafe { hs.borrow() }.expect("Couldn't unwrap h5_state");
        let offset = unsafe { hs.token_start.offset(-(hs.s as isize)) } as isize;
        let segment = unsafe { ::std::slice::from_raw_parts(hs.s.offset(offset), hs.token_len) };
        //let segment = String::from_utf8_lossy(segment);
        let type_ = h5_type_to_string(hs.token_type);

        TestTriple { type_: hs.token_type, len: hs.token_len, value: segment }
    }

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
            token_type: html5_type::DATA_TEXT,
        };

        let test_html = "<script>alert(document.domain)</script>";
        let hs_ptr = hs.borrow_mut() as *mut h5_state;


        libinjection_h5_init(hs_ptr, test_html.as_ptr() as *const u8, test_html.len(), html5_flags::DATA_STATE);
        while libinjection_h5_next(&mut hs) == 1 {
            print_html5_token(hs_ptr);
        }
    }

    #[test]
    fn test_html_parse_safe() {
        let test_html = "<script>alert(documentXdomain)</script>";
        let mut hs_safe = libinjection_h5_init_safe(test_html.as_bytes(), html5_flags::DATA_STATE);
        while libinjection_h5_next_safe(&mut hs_safe) == 1 {
            print_html5_token_safe(&hs_safe);
        }
    }

    #[derive(PartialEq, Debug)]
    struct TestTriple<'a> {
        type_: html5_type,
        len: usize,
        value: &'a [u8],
    }


    /**
     * tag with name starting with '='
     */
    #[test]
    fn test_tag_with_name_starting_with_equals_() {
        let input = "<foo =_=xxx";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 2, value: "=_".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "xxx".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * XML/HTML Entity definition
     */
    #[test]
    fn test_XML_HTML_Entity_definition() {
        let input = "<!ENTITY foo>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 10, value: "ENTITY foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, unquoted value, spaces
     */
    #[test]
    fn test_tag_with_name_attribute_unquoted_value_spaces01() {
        let input = "<foo  bar= \"yes\" >";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "yes".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * doctype, lowercase
     */
    #[test]
    fn test_doctype_lowercase() {
        let input = "<!doctype>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DOCTYPE, len: 7, value: "doctype".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * comments with dashess and bangs

     There's a bug in test-html5-040.txt
     The expected string length is 10 bytes, but the expected string is only 9bytes ' -x -- -!'

     I'm making the expected string ' -x -- -! '


     */
    #[test]
    fn test_comments_with_dashess_and_bangs01() {
        let input = "<!-- -x -- -! -->";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 10, value: " -x -- -! ".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * comments with dashess and bangs
     */
    #[test]
    fn test_comments_with_dashess_and_bangs04() {
        let input = "<!------->";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 3, value: "---".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }


    /**
     * tags
     */
    #[test]
    fn test_tags01() {
        let input = "<foo/";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, quoted value, unclosed
     */
    #[test]
    fn test_tag_with_name_attribute_quoted_value_unclosed01() {
        let input = "<foo  bar = 'xxx";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "xxx".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * Basic doctype
     */
    #[test]
    fn test_Basic_doctype() {
        let input = "<!DOCTYPE>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DOCTYPE, len: 7, value: "DOCTYPE".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * cdata, degenerate
     */
    #[test]
    fn test_cdata_degenerate01() {
        let input = "<?import foo=\"bar\"/>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 17, value: "import foo=\"bar\"/".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * non-html tag
     */
    #[test]
    fn test_non_html_tag() {
        let input = "<1234 foo";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 1, value: "<".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 8, value: "1234 foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, quoted value, unclosed
     */
    #[test]
    fn test_tag_with_name_attribute_quoted_value_unclosed02() {
        let input = "<foo bar/>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_SELFCLOSE, len: 2, value: "/>".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * comments with dashess and bangs
     */
    #[test]
    fn test_comments_with_dashess_and_bangs02() {
        let input = "<foo /junk>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 4, value: "junk".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * doctype with other stuff, eof
     */
    #[test]
    fn test_doctype_with_other_stuff_eof() {
        let input = "<!DOCTYPE \"stuff\"";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DOCTYPE, len: 15, value: "DOCTYPE \"stuff\"".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tags
     */
    #[test]
    fn test_tags02() {
        let input = "<foo>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * comments with dashess and bangs
     */
    #[test]
    fn test_comments_with_dashess_and_bangs03() {
        let input = "<!--foo--";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 5, value: "foo--".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, quoted value, unclosed, more whitespace
     */
    #[test]
    fn test_tag_with_name_attribute_quoted_value_unclosed_more_whitespace() {
        let input = "<foo  bar   =   \"xxx\"";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "xxx".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tags
     */
    #[test]
    fn test_tags03() {
        let input = "<foo";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tags
     */
    #[test]
    fn test_tags04() {
        let input = "<foo/>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_SELFCLOSE, len: 2, value: "/>".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tags
     */
    #[test]
    fn test_tags05() {
        let input = "aa<foo>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 2, value: "aa".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * attribute name right after quoted value with no whitespace
     */
    #[test]
    fn test_attribute_name_right_after_quoted_value_with_no_whitespace() {
        let input = "<foo bar=\"yes\"isdir>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "yes".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 5, value: "isdir".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * cdata, degenerate
     */
    #[test]
    fn test_cdata_degenerate04() {
        let input = "<![CDATA[foobar";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 6, value: "foobar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, unquoted value, trailing spaces, EOF
     */
    #[test]
    fn test_tag_with_name_attribute_unquoted_value_trailing_spaces_EOF() {
        let input = "<foo  bar=yes";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "yes".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * self-close, eof before final closed
     */
    #[test]
    fn test_self_close_eof_before_final_closed() {
        let input = "<foo   /";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, tailing spaces, EOF
     */
    #[test]
    fn test_tag_with_name_attribute_tailing_spaces_EOF() {
        let input = "<foo  bar";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, unquoted value, EOF
     */
    #[test]
    fn test_tag_with_name_attribute_unquoted_value_EOF() {
        let input = "<foo  bar=yes";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "yes".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * bogus comment, with EOF
     */
    #[test]
    fn test_bogus_comment_with_EOF() {
        let input = "<?foo";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, quoted value, unclosed
     */
    #[test]
    fn test_tag_with_name_attribute_quoted_value_unclosed04() {
        let input = "</foo>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_CLOSE, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * ascii word in double quotes
     */
    #[test]
    fn test_ascii_word_in_double_quotes() {
        let input = "\"foo\"";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 5, value: "\"foo\"".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, unquoted value, spaces
     */
    #[test]
    fn test_tag_with_name_attribute_unquoted_value_spaces02() {
        let input = "<foo  bar =  >";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 0, value: "".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with since name attribute
     */
    #[test]
    fn test_tag_with_since_name_attribute() {
        let input = "<foo  bar>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, unquoted value
     */
    #[test]
    fn test_tag_with_name_attribute_unquoted_value() {
        let input = "<foo  bar=yes>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "yes".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * ascii word
     */
    #[test]
    fn test_ascii_word() {
        let input = "foo";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * cdata, degenerate
     */
    #[test]
    fn test_cdata_degenerate06() {
        let input = "<![CDATA foo>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 10, value: "[CDATA foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tags,- 'foo'
     */
    #[test]
    fn test_tags_foo_() {
        let input = "<foo>bb";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 2, value: "bb".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * cdata, degenerate
     */
    #[test]
    fn test_cdata_degenerate07() {
        let input = "<![CDATA[foobar";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 6, value: "foobar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * bogus comment
     */
    #[test]
    fn test_bogus_comment() {
        let input = "<?foo>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
   * tags
   */
    #[test]
    fn test_tags06() {
        let input = "<foo          >";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
   * tag with name attribute, quoted value, unclosed
   */
    #[test]
    fn test_tag_with_name_attribute_quoted_value_unclosed05() {
        let input = "<foo  bar = \"xxx\"";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "xxx".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
   * tag with name attribute, unquoted value, spaces
   */
    #[test]
    fn test_tag_with_name_attribute_unquoted_value_spaces03() {
        let input = "<foo  bar= 'yes' >";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "yes".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
   * cdata, degenerate
   */
    #[test]
    fn test_cdata_degenerate08() {
        let input = "<![CDATAX foo>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 11, value: "[CDATAX foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
   * doctype with other stuff
   */
    #[test]
    fn test_doctype_with_other_stuff() {
        let input = "<!DOCTYPE \"stuff\">";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DOCTYPE, len: 15, value: "DOCTYPE \"stuff\"".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
   * tags
   */
    #[test]
    fn test_tags07() {
        let input = "aa<foo>bb";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 2, value: "aa".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 2, value: "bb".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
   * tag with name attribute, back-tick value
   */
    #[test]
    fn test_tag_with_name_attribute_back_tick_value() {
        let input = "<foo  bar   =   `xxx`";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "xxx".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
   * tag with name attribute, quoted value, unclosed
   */
    #[test]
    fn test_tag_with_name_attribute_quoted_value_unclosed06() {
        let input = "<foo  bar = '";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 0, value: "".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
    * empty input string, expect nothing
    */
    #[test]
    fn test_empty_input_string_expect_nothing() {
        let input = "";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);

        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
   * gt after quoted attribute value
   */
    #[test]
    fn test_gt_after_quoted_attribute_value() {
        let input = "<foo bar=\"yes\">";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "yes".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }


    /**
     * tag with name attribute, unquoted value, spaces
     */
    #[test]
    fn test_tag_with_name_attribute_unquoted_value_spaces04() {
        let input = "<foo  bar = '' >";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 0, value: "".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tags
     */
    #[test]
    fn test_tags08() {
        let input = "<foo";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, unquoted value, spaces
     */
    #[test]
    fn test_tag_with_name_attribute_unquoted_value_spaces05() {
        let input = "<foo  bar=>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 0, value: "".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tags
     */
    #[test]
    fn test_tags09() {
        let input = "<foo   />";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_SELFCLOSE, len: 2, value: "/>".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * slash right after quoted attribute value
     */
    #[test]
    fn test_slash_right_after_quoted_attribute_value() {
        let input = "<foo bar=\"yes\"/>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "yes".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_SELFCLOSE, len: 2, value: "/>".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, quoted value, unclosed
     */
    #[test]
    fn test_tag_with_name_attribute_quoted_value_unclosed07() {
        let input = "<!--foo-->";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, EOF
     */
    #[test]
    fn test_tag_with_name_attribute_EOF() {
        let input = "<foo  bar";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, quoted value, unclosed
     */
    #[test]
    fn test_tag_with_name_attribute_quoted_value_unclosed08() {
        let input = "</foo  >";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * tag with name attribute, unquoted value, trailing spaces
     */
    #[test]
    fn test_tag_with_name_attribute_unquoted_value_trailing_spaces() {
        let input = "<foo  bar=yes  >";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_NAME, len: 3, value: "bar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::ATTR_VALUE, len: 3, value: "yes".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }


    /**
     * tag with name attribute, quoted value, unclosed
     */
    #[test]
    fn test_tag_with_name_attribute_quoted_value_unclosed03() {
        let input = "<!--foo";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 3, value: "foo".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }


    /**
    * alternate <% comment %> format used by IE and old safari
    */
    #[test]
    fn test_alternate_comment_format_used_by_IE_and_old_safari() {
        let input = "<% foo><x foo=\"%><script>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 13, value: " foo><x foo=\"".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 6, value: "script".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * cdata, degenerate
     */
    #[test]
    fn test_cdata_degenerate02() {
        let input = "<![CDATA";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 6, value: "[CDATA".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * cdata
     */
    #[test]
    fn test_cdata() {
        let input = "<![CDATA[foobar]]>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 6, value: "foobar".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }


    /**
 * cdata, degenerate
 */
    #[test]
    fn test_cdata_degenerate03() {
        let input = "<![CDATA";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 6, value: "[CDATA".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * cdata, degenerate
     */
    #[test]
    fn test_cdata_degenerate05() {
        let input = "<![CDATA[foobar]]]>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::DATA_TEXT, len: 7, value: "foobar]".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }

    /**
     * bogus closing tags
     */
    #[test]
    fn test_bogus_closing_tags() {
        let input = "</ foo=\"><script>";
        let mut hs = test_init(input.as_bytes(), html5_flags::DATA_STATE);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_COMMENT, len: 6, value: " foo=\"".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_OPEN, len: 6, value: "script".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 1);
        let actual = convert(hs.clone());
        assert_eq!(TestTriple { type_: html5_type::TAG_NAME_CLOSE, len: 1, value: ">".as_bytes() }, actual);
        assert_eq!(test_next_token(&mut hs), 0);
    }
}
