extern crate libc;
extern crate injection;
use injection::libinjection_xss::libinjection_xss_safe;

mod ffi {
    extern {
        pub fn clock() -> ::libc::clock_t;
    }
}


fn test_is_xss() -> i32 {
    let s = [
        "<script>alert(1);</script>".as_bytes(),
        "><script>alert(1);</script>".as_bytes(),
        "x ><script>alert(1);</script>".as_bytes(),
        "' ><script>alert(1);</script>".as_bytes(),
        "\"><script>alert(1);</script>".as_bytes(),
        "red;</style><script>alert(1);</script>".as_bytes(),
        "red;}</style><script>alert(1);</script>".as_bytes(),
        "red;\"/><script>alert(1);</script>".as_bytes(),
        "');}</style><script>alert(1);</script>".as_bytes(),
        "onerror=alert(1)>".as_bytes(),
        "x onerror=alert(1);>".as_bytes(),
        "x' onerror=alert(1);>".as_bytes(),
        "x\" onerror=alert(1);>".as_bytes(),
        "<a href=\"javascript:alert(1)\">".as_bytes(),
        "<a href='javascript:alert(1)'>".as_bytes(),
        "<a href=javascript:alert(1)>".as_bytes(),
        "<a href  =   javascript:alert(1); >".as_bytes(),
        "<a href=\"  javascript:alert(1);\" >".as_bytes(),
        "<a href=\"JAVASCRIPT:alert(1);\" >".as_bytes(),
        "123 LIKE -1234.5678E+2;".as_bytes(),
        "APPLE 19.123 'FOO' \"BAR\"".as_bytes(),
        "/* BAR */ UNION ALL SELECT (2,3,4)".as_bytes(),
        "1 || COS(+0X04) --FOOBAR".as_bytes(),
        "dog apple @cat banana bar".as_bytes(),
        "dog apple cat \"banana \'bar".as_bytes(),
        "102 TABLE CLOTH".as_bytes(),
        "(1001-'1') union select 1,2,3,4 from credit_cards".as_bytes(),
    ];


    let imax: usize = 10000000usize;
    let mut _dont_optimize_me_bro;
    let t0 = unsafe {ffi::clock() };
    let len = s.len();
    let mut i: usize = 0;

    'loop1: loop {
        if i == imax {
            break;
        }
        _dont_optimize_me_bro = libinjection_xss_safe(s[i % len ]);
        i = i + 1;
    }

    let t1 = unsafe {ffi::clock() };
    let total: f64  = (t1 - t0) as (f64) / 1000000 as (f64);
    println!("iters: {}", i);
     (imax as (f64) / total) as (i32)
}

fn main()  {
    let mintps: i32 = 500000i32;
    let tps: i32 = test_is_xss();
    println!("\nTPS : {}\n\n", tps);
    if tps < 500000i32 {
        println!("OK: {} < {}\n", tps, mintps);
        //1i32
    } else {
        println!("OK: {} > {}\n", tps, mintps);
        //0i32
    }
}
