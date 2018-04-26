extern {
    fn libinjection_h5_init(
        hs : *mut h5_state, s : *const u8, len : usize, arg4 : html5_flags
    );
    fn libinjection_h5_next(hs : *mut h5_state) -> i32;
    fn memchr(
        __s : *const ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
}

static mut gsHexDecodeMap
    : [i32; 256]
    = [   256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          0i32,
          1i32,
          2i32,
          3i32,
          4i32,
          5i32,
          6i32,
          7i32,
          8i32,
          9i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          10i32,
          11i32,
          12i32,
          13i32,
          14i32,
          15i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          10i32,
          11i32,
          12i32,
          13i32,
          14i32,
          15i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32,
          256i32
      ];

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum attribute {
    TYPE_NONE,
    TYPE_BLACK,
    TYPE_ATTR_URL,
    TYPE_STYLE,
    TYPE_ATTR_INDIRECT,
}

#[derive(Copy)]
#[repr(C)]
pub struct stringtype {
    pub name : *const u8,
    pub atype : attribute,
}

impl Clone for stringtype {
    fn clone(&self) -> Self { *self }
}

static mut BLACKATTR : *mut stringtype = 0 as (*mut stringtype);

static mut BLACKTAG : *mut *const u8 = 0 as (*mut *const u8);

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

unsafe extern fn cstrcasecmp_with_null(
    mut a : *const u8, mut b : *const u8, mut n : usize
) -> i32 {
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
            _currentBlock = 9;
            break;
        }
    }
    if _currentBlock == 2 {
        (if *a as (i32) == 0i32 { 0i32 } else { 1i32 })
    } else {
        1i32
    }
}

unsafe extern fn html_decode_char_at(
    mut src : *const u8, mut len : usize, mut consumed : *mut usize
) -> i32 {
    let mut _currentBlock;
    let mut val : i32 = 0i32;
    let mut i : usize;
    let mut ch : i32;
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

unsafe extern fn htmlencode_startswith(
    mut a : *const u8, mut b : *const u8, mut n : usize
) -> i32 {
    let mut _currentBlock;
    let mut consumed : usize;
    let mut cb : i32;
    let mut first : i32 = 1i32;
    'loop1: loop {
        if !(n > 0usize) {
            _currentBlock = 2;
            break;
        }
        if *a as (i32) == 0i32 {
            _currentBlock = 12;
            break;
        }
        cb = html_decode_char_at(b,n,&mut consumed as (*mut usize));
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
            _currentBlock = 11;
            break;
        }
        a = a.offset(1isize);
    }
    if _currentBlock == 2 {
        (if *a as (i32) == 0i32 { 1i32 } else { 0i32 })
    } else if _currentBlock == 11 {
        0i32
    } else {
        1i32
    }
}

unsafe extern fn is_black_url(
    mut s : *const u8, mut len : usize
) -> i32 {
    static mut data_url : *const u8 = (*b"DATA\0").as_ptr();
    static mut viewsource_url
        : *const u8
        = (*b"VIEW-SOURCE\0").as_ptr();
    static mut vbscript_url : *const u8 = (*b"VBSCRIPT\0").as_ptr();
    static mut javascript_url : *const u8 = (*b"JAVA\0").as_ptr();
    'loop1: loop {
        if !(len > 0usize && (*s as (i32) <= 32i32 || *s as (i32) >= 127i32)) {
            break;
        }
        s = s.offset(1isize);
        len = len.wrapping_sub(1usize);
    }
    if htmlencode_startswith(data_url,s,len) != 0 {
        1i32
    } else if htmlencode_startswith(viewsource_url,s,len) != 0 {
        1i32
    } else if htmlencode_startswith(javascript_url,s,len) != 0 {
        1i32
    } else if htmlencode_startswith(vbscript_url,s,len) != 0 {
        1i32
    } else {
        0i32
    }
}

unsafe extern fn is_black_attr(
    mut s : *const u8, mut len : usize
) -> attribute {
    let mut _currentBlock;
    let mut black : *mut stringtype;
    if len < 2usize {
        attribute::TYPE_NONE
    } else {
        if len >= 5usize {
            if (*s.offset(0isize) as (i32) == b'o' as (i32) || *s.offset(
                                                                    0isize
                                                                ) as (i32) == b'O' as (i32)) && (*s.offset(
                                                                                                      1isize
                                                                                                  ) as (i32) == b'n' as (i32) || *s.offset(
                                                                                                                                      1isize
                                                                                                                                  ) as (i32) == b'N' as (i32)) {
                return attribute::TYPE_BLACK;
            } else if cstrcasecmp_with_null(
                          (*b"XMLNS\0").as_ptr(),
                          s,
                          5usize
                      ) == 0i32 || cstrcasecmp_with_null(
                                       (*b"XLINK\0").as_ptr(),
                                       s,
                                       5usize
                                   ) == 0i32 {
                return attribute::TYPE_BLACK;
            }
        }
        black = BLACKATTR;
        'loop5: loop {
            if !((*black).name != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8)) {
                _currentBlock = 6;
                break;
            }
            if cstrcasecmp_with_null((*black).name,s,len) == 0i32 {
                _currentBlock = 9;
                break;
            }
            black = black.offset(1isize);
        }
        (if _currentBlock == 6 {
             attribute::TYPE_NONE
         } else {
             (*black).atype
         })
    }
}

unsafe extern fn is_black_tag(
    mut s : *const u8, mut len : usize
) -> i32 {
    let mut _currentBlock;
    let mut black : *mut *const u8;
    if len < 3usize {
        0i32
    } else {
        black = BLACKTAG;
        'loop2: loop {
            if !(*black != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8)) {
                _currentBlock = 3;
                break;
            }
            if cstrcasecmp_with_null(*black,s,len) == 0i32 {
                _currentBlock = 10;
                break;
            }
            black = black.offset(1isize);
        }
        (if _currentBlock == 3 {
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
pub unsafe extern fn libinjection_is_xss(
    mut s : *const u8, mut len : usize, mut flags : i32
) -> i32 {
    let mut _currentBlock;
    let mut h5 : h5_state;
    let mut attr : attribute = attribute::TYPE_NONE;
    libinjection_h5_init(
        &mut h5 as (*mut h5_state),
        s,
        len,
        flags as (html5_flags)
    );
    'loop1: loop {
        if libinjection_h5_next(&mut h5 as (*mut h5_state)) == 0 {
            _currentBlock = 2;
            break;
        }
        if h5.token_type as (i32) != html5_type::ATTR_VALUE as (i32) {
            attr = attribute::TYPE_NONE;
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
            if !(attr as (i32) == attribute::TYPE_NONE as (i32)) {
                if attr as (i32) == attribute::TYPE_ATTR_INDIRECT as (i32) {
                    if is_black_attr(h5.token_start,h5.token_len) != 0 {
                        _currentBlock = 32;
                        break;
                    }
                } else {
                    if attr as (i32) == attribute::TYPE_STYLE as (i32) {
                        _currentBlock = 30;
                        break;
                    }
                    if attr as (i32) == attribute::TYPE_ATTR_URL as (i32) {
                        if is_black_url(h5.token_start,h5.token_len) != 0 {
                            _currentBlock = 29;
                            break;
                        }
                    } else if attr as (i32) == attribute::TYPE_BLACK as (i32) {
                        _currentBlock = 27;
                        break;
                    }
                }
            }
            attr = attribute::TYPE_NONE;
        } else {
            if !(h5.token_type as (i32) == html5_type::TAG_COMMENT as (i32)) {
                continue;
            }
            if memchr(
                   h5.token_start as (*const ::std::os::raw::c_void),
                   b'`' as (i32),
                   h5.token_len
               ) != 0i32 as (*mut ::std::os::raw::c_void) {
                _currentBlock = 21;
                break;
            }
            if h5.token_len > 3usize {
                if *h5.token_start.offset(
                        0isize
                    ) as (i32) == b'[' as (i32) && (*h5.token_start.offset(
                                                         1isize
                                                     ) as (i32) == b'i' as (i32) || *h5.token_start.offset(
                                                                                         1isize
                                                                                     ) as (i32) == b'I' as (i32)) && (*h5.token_start.offset(
                                                                                                                           2isize
                                                                                                                       ) as (i32) == b'f' as (i32) || *h5.token_start.offset(
                                                                                                                                                           2isize
                                                                                                                                                       ) as (i32) == b'F' as (i32)) {
                    _currentBlock = 20;
                    break;
                }
                if (*h5.token_start.offset(
                         0isize
                     ) as (i32) == b'x' as (i32) || *h5.token_start.offset(
                                                         0isize
                                                     ) as (i32) == b'X' as (i32)) && (*h5.token_start.offset(
                                                                                           1isize
                                                                                       ) as (i32) == b'm' as (i32) || *h5.token_start.offset(
                                                                                                                           1isize
                                                                                                                       ) as (i32) == b'M' as (i32)) && (*h5.token_start.offset(
                                                                                                                                                             2isize
                                                                                                                                                         ) as (i32) == b'l' as (i32) || *h5.token_start.offset(
                                                                                                                                                                                             2isize
                                                                                                                                                                                         ) as (i32) == b'L' as (i32)) {
                    _currentBlock = 19;
                    break;
                }
            }
            if !(h5.token_len > 5usize) {
                continue;
            }
            if cstrcasecmp_with_null(
                   (*b"IMPORT\0").as_ptr(),
                   h5.token_start,
                   6usize
               ) == 0i32 {
                _currentBlock = 18;
                break;
            }
            if cstrcasecmp_with_null(
                   (*b"ENTITY\0").as_ptr(),
                   h5.token_start,
                   6usize
               ) == 0i32 {
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
pub unsafe extern fn libinjection_xss(
    mut s : *const u8, mut len : usize
) -> i32 {
    if libinjection_is_xss(
           s,
           len,
           html5_flags::DATA_STATE as (i32)
       ) != 0 {
        1i32
    } else if libinjection_is_xss(
                  s,
                  len,
                  html5_flags::VALUE_NO_QUOTE as (i32)
              ) != 0 {
        1i32
    } else if libinjection_is_xss(
                  s,
                  len,
                  html5_flags::VALUE_SINGLE_QUOTE as (i32)
              ) != 0 {
        1i32
    } else if libinjection_is_xss(
                  s,
                  len,
                  html5_flags::VALUE_DOUBLE_QUOTE as (i32)
              ) != 0 {
        1i32
    } else if libinjection_is_xss(
                  s,
                  len,
                  html5_flags::VALUE_BACK_QUOTE as (i32)
              ) != 0 {
        1i32
    } else {
        0i32
    }
}
