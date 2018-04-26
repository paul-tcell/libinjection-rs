extern {
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn memchr(
        __s : *const ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn strchr(__s : *const u8, __c : i32) -> *mut u8;
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
pub unsafe extern fn libinjection_h5_init(
    mut hs : *mut h5_state,
    mut s : *const u8,
    mut len : usize,
    mut flags : html5_flags
) {
    memset(
        hs as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<h5_state>()
    );
    (*hs).s = s;
    (*hs).len = len;
    if flags as (i32) == html5_flags::VALUE_BACK_QUOTE as (i32) {
        (*hs).state = h5_state_attribute_value_back_quote as (unsafe extern fn(*mut h5_state) -> i32);
    } else if flags as (i32) == html5_flags::VALUE_DOUBLE_QUOTE as (i32) {
        (*hs).state = h5_state_attribute_value_double_quote as (unsafe extern fn(*mut h5_state) -> i32);
    } else if flags as (i32) == html5_flags::VALUE_SINGLE_QUOTE as (i32) {
        (*hs).state = h5_state_attribute_value_single_quote as (unsafe extern fn(*mut h5_state) -> i32);
    } else if flags as (i32) == html5_flags::VALUE_NO_QUOTE as (i32) {
        (*hs).state = h5_state_before_attribute_name as (unsafe extern fn(*mut h5_state) -> i32);
    } else if flags as (i32) == html5_flags::DATA_STATE as (i32) {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
    }
}

#[no_mangle]
pub unsafe extern fn libinjection_h5_next(
    mut hs : *mut h5_state
) -> i32 {
    if (*hs).state as (*mut ::std::os::raw::c_void) != 0i32 as (*mut ::std::os::raw::c_void) {
        0i32;
    } else {
        __assert_fail(
            (*b"hs->state != NULL\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"libinjection_h5_next\0").as_ptr()
        );
    }
    ((*hs).state)(hs as (*mut h5_state))
}

unsafe extern fn h5_state_eof(mut hs : *mut h5_state) -> i32 {
    hs;
    0i32
}

unsafe extern fn h5_state_data(mut hs : *mut h5_state) -> i32 {
    let mut idx : *const u8;
    if (*hs).len >= (*hs).pos {
        0i32;
    } else {
        __assert_fail(
            (*b"hs->len >= hs->pos\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"h5_state_data\0").as_ptr()
        );
    }
    idx = memchr(
              (*hs).s.offset(
                  (*hs).pos as (isize)
              ) as (*const ::std::os::raw::c_void),
              60i32,
              (*hs).len.wrapping_sub((*hs).pos)
          ) as (*const u8);
    if idx == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
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
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / ::std::mem::size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
                          (*hs).s as (isize)
                      ) / ::std::mem::size_of::<u8>(
                          ) as (isize)) as (usize)).wrapping_add(
                        1usize
                    );
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

unsafe extern fn h5_state_end_tag_open(
    mut hs : *mut h5_state
) -> i32 {
    let mut ch : u8;
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

unsafe extern fn h5_is_white(mut ch : u8) -> i32 {
    (strchr(
         (*b" \t\n\x0B\x0C\r\0").as_ptr(),
         ch as (i32)
     ) != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) as (i32)
}

unsafe extern fn h5_state_tag_name_close(
    mut hs : *mut h5_state
) -> i32 {
    (*hs).is_close = 0i32;
    (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
    (*hs).token_len = 1usize;
    (*hs).token_type = html5_type::TAG_NAME_CLOSE;
    (*hs).pos = (*hs).pos.wrapping_add(1usize);
    if (*hs).pos < (*hs).len {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
    } else {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
    }
    1i32
}

unsafe extern fn h5_state_tag_name(mut hs : *mut h5_state) -> i32 {
    let mut _currentBlock;
    let mut ch : u8;
    let mut pos : usize;
    pos = (*hs).pos;
    'loop1: loop {
        if !(pos < (*hs).len) {
            _currentBlock = 2;
            break;
        }
        ch = *(*hs).s.offset(pos as (isize));
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
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::TAG_NAME_OPEN;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    } else if _currentBlock == 8 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        if (*hs).is_close != 0 {
            (*hs).pos = pos.wrapping_add(1usize);
            (*hs).is_close = 0i32;
            (*hs).token_type = html5_type::TAG_CLOSE;
            (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        } else {
            (*hs).pos = pos;
            (*hs).token_type = html5_type::TAG_NAME_OPEN;
            (*hs).state = h5_state_tag_name_close as (unsafe extern fn(*mut h5_state) -> i32);
        }
        1i32
    } else if _currentBlock == 12 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::TAG_NAME_OPEN;
        (*hs).pos = pos.wrapping_add(1usize);
        (*hs).state = h5_state_self_closing_start_tag as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::TAG_NAME_OPEN;
        (*hs).pos = pos.wrapping_add(1usize);
        (*hs).state = h5_state_before_attribute_name as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    }
}

unsafe extern fn h5_skip_white(mut hs : *mut h5_state) -> i32 {
    let mut _currentBlock;
    let mut ch : u8;
    'loop1: loop {
        if !((*hs).pos < (*hs).len) {
            _currentBlock = 2;
            break;
        }
        ch = *(*hs).s.offset((*hs).pos as (isize));
        if !(ch as (i32) == 0xdi32 || ch as (i32) == 0xci32 || ch as (i32) == 0xbi32 || ch as (i32) == 0xai32 || ch as (i32) == 0x9i32 || ch as (i32) == 0x20i32 || ch as (i32) == 0x0i32) {
            _currentBlock = 5;
            break;
        }
        (*hs).pos = (*hs).pos.wrapping_add(1usize);
    }
    if _currentBlock == 2 { -1i32 } else { ch as (i32) }
}

unsafe extern fn h5_state_before_attribute_name(
    mut hs : *mut h5_state
) -> i32 {
    let mut ch : i32;
    ch = h5_skip_white(hs);
    if ch == 62i32 {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = 1usize;
        (*hs).token_type = html5_type::TAG_NAME_CLOSE;
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
    mut hs : *mut h5_state
) -> i32 {
    let mut _currentBlock;
    let mut ch : u8;
    let mut pos : usize;
    pos = (*hs).pos.wrapping_add(1usize);
    'loop1: loop {
        if !(pos < (*hs).len) {
            _currentBlock = 2;
            break;
        }
        ch = *(*hs).s.offset(pos as (isize));
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
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::ATTR_NAME;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).pos = (*hs).len;
        1i32
    } else if _currentBlock == 8 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::ATTR_NAME;
        (*hs).state = h5_state_tag_name_close as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).pos = pos;
        1i32
    } else if _currentBlock == 9 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::ATTR_NAME;
        (*hs).state = h5_state_before_attribute_value as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).pos = pos.wrapping_add(1usize);
        1i32
    } else if _currentBlock == 10 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::ATTR_NAME;
        (*hs).state = h5_state_self_closing_start_tag as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).pos = pos.wrapping_add(1usize);
        1i32
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::ATTR_NAME;
        (*hs).state = h5_state_after_attribute_name as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).pos = pos.wrapping_add(1usize);
        1i32
    }
}

unsafe extern fn h5_state_after_attribute_name(
    mut hs : *mut h5_state
) -> i32 {
    let mut c : i32;
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
    mut hs : *mut h5_state
) -> i32 {
    let mut c : i32;
    c = h5_skip_white(hs);
    if c == -1i32 {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
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

unsafe extern fn h5_state_attribute_value_quote(
    mut hs : *mut h5_state, mut qchar : u8
) -> i32 {
    let mut idx : *const u8;
    if (*hs).pos > 0usize {
        (*hs).pos = (*hs).pos.wrapping_add(1usize);
    }
    idx = memchr(
              (*hs).s.offset(
                  (*hs).pos as (isize)
              ) as (*const ::std::os::raw::c_void),
              qchar as (i32),
              (*hs).len.wrapping_sub((*hs).pos)
          ) as (*const u8);
    if idx == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::ATTR_VALUE;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / ::std::mem::size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).token_type = html5_type::ATTR_VALUE;
        (*hs).state = h5_state_after_attribute_value_quoted_state as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).pos = (*hs).pos.wrapping_add(
                        (*hs).token_len.wrapping_add(1usize)
                    );
    }
    1i32
}

unsafe extern fn h5_state_attribute_value_double_quote(
    mut hs : *mut h5_state
) -> i32 {
    h5_state_attribute_value_quote(hs,34u8)
}

unsafe extern fn h5_state_attribute_value_single_quote(
    mut hs : *mut h5_state
) -> i32 {
    h5_state_attribute_value_quote(hs,39u8)
}

unsafe extern fn h5_state_attribute_value_back_quote(
    mut hs : *mut h5_state
) -> i32 {
    h5_state_attribute_value_quote(hs,96u8)
}

unsafe extern fn h5_state_attribute_value_no_quote(
    mut hs : *mut h5_state
) -> i32 {
    let mut _currentBlock;
    let mut ch : u8;
    let mut pos : usize;
    pos = (*hs).pos;
    'loop1: loop {
        if !(pos < (*hs).len) {
            _currentBlock = 2;
            break;
        }
        ch = *(*hs).s.offset(pos as (isize));
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
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).token_type = html5_type::ATTR_VALUE;
        1i32
    } else if _currentBlock == 6 {
        (*hs).token_type = html5_type::ATTR_VALUE;
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).pos = pos;
        (*hs).state = h5_state_tag_name_close as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    } else {
        (*hs).token_type = html5_type::ATTR_VALUE;
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = pos.wrapping_sub((*hs).pos);
        (*hs).pos = pos.wrapping_add(1usize);
        (*hs).state = h5_state_before_attribute_name as (unsafe extern fn(*mut h5_state) -> i32);
        1i32
    }
}

unsafe extern fn h5_state_after_attribute_value_quoted_state(
    mut hs : *mut h5_state
) -> i32 {
    let mut ch : u8;
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
             (*hs).token_type = html5_type::TAG_NAME_CLOSE;
             (*hs).pos = (*hs).pos.wrapping_add(1usize);
             (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
             1i32
         } else {
             h5_state_before_attribute_name(hs)
         })
    }
}

unsafe extern fn h5_state_self_closing_start_tag(
    mut hs : *mut h5_state
) -> i32 {
    let mut ch : u8;
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
                     (*b"h5_state_self_closing_start_tag\0").as_ptr()
                 );
             }
             (*hs).token_start = (*hs).s.offset((*hs).pos as (isize)).offset(
                                     -1isize
                                 );
             (*hs).token_len = 2usize;
             (*hs).token_type = html5_type::TAG_NAME_SELFCLOSE;
             (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
             (*hs).pos = (*hs).pos.wrapping_add(1usize);
             1i32
         } else {
             h5_state_before_attribute_name(hs)
         })
    }
}

unsafe extern fn h5_state_bogus_comment(
    mut hs : *mut h5_state
) -> i32 {
    let mut idx : *const u8;
    idx = memchr(
              (*hs).s.offset(
                  (*hs).pos as (isize)
              ) as (*const ::std::os::raw::c_void),
              62i32,
              (*hs).len.wrapping_sub((*hs).pos)
          ) as (*const u8);
    if idx == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
        (*hs).pos = (*hs).len;
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
    } else {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / ::std::mem::size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
                          (*hs).s as (isize)
                      ) / ::std::mem::size_of::<u8>(
                          ) as (isize)) as (usize)).wrapping_add(
                        1usize
                    );
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
    }
    (*hs).token_type = html5_type::TAG_COMMENT;
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
                  (*hs).s.offset(pos as (isize)) as (*const ::std::os::raw::c_void),
                  37i32,
                  (*hs).len.wrapping_sub(pos)
              ) as (*const u8);
        if idx == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) || idx.offset(
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
                ) / ::std::mem::size_of::<u8>(
                    ) as (isize)) as (usize)).wrapping_add(
                  1usize
              );
    }
    if _currentBlock == 3 {
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / ::std::mem::size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
                          (*hs).s as (isize)
                      ) / ::std::mem::size_of::<u8>(
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

unsafe extern fn h5_state_comment(mut hs : *mut h5_state) -> i32 {
    let mut _currentBlock;
    let mut ch : u8;
    let mut idx : *const u8;
    let mut pos : usize;
    let mut offset : usize;
    let mut end : *const u8 = (*hs).s.offset((*hs).len as (isize));
    pos = (*hs).pos;
    'loop1: loop {
        idx = memchr(
                  (*hs).s.offset(pos as (isize)) as (*const ::std::os::raw::c_void),
                  45i32,
                  (*hs).len.wrapping_sub(pos)
              ) as (*const u8);
        if idx == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) || idx > (*hs).s.offset(
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
                    ) / ::std::mem::size_of::<u8>(
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
                    ) / ::std::mem::size_of::<u8>(
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
                            ) / ::std::mem::size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = ((idx.offset(
                          offset as (isize)
                      ) as (isize)).wrapping_sub(
                         (*hs).s as (isize)
                     ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
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
                  (*hs).s.offset(pos as (isize)) as (*const ::std::os::raw::c_void),
                  93i32,
                  (*hs).len.wrapping_sub(pos)
              ) as (*const u8);
        if idx == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) || idx > (*hs).s.offset(
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
                ) / ::std::mem::size_of::<u8>(
                    ) as (isize)) as (usize)).wrapping_add(
                  1usize
              );
    }
    if _currentBlock == 4 {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_start = (*hs).s.offset((*hs).pos as (isize));
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / ::std::mem::size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
                          (*hs).s as (isize)
                      ) / ::std::mem::size_of::<u8>(
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
              ) as (*const ::std::os::raw::c_void),
              62i32,
              (*hs).len.wrapping_sub((*hs).pos)
          ) as (*const u8);
    if idx == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        (*hs).state = h5_state_eof as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_len = (*hs).len.wrapping_sub((*hs).pos);
    } else {
        (*hs).state = h5_state_data as (unsafe extern fn(*mut h5_state) -> i32);
        (*hs).token_len = (((idx as (isize)).wrapping_sub(
                                (*hs).s as (isize)
                            ) / ::std::mem::size_of::<u8>(
                                ) as (isize)) as (usize)).wrapping_sub(
                              (*hs).pos
                          );
        (*hs).pos = (((idx as (isize)).wrapping_sub(
                          (*hs).s as (isize)
                      ) / ::std::mem::size_of::<u8>(
                          ) as (isize)) as (usize)).wrapping_add(
                        1usize
                    );
    }
    1i32
}
