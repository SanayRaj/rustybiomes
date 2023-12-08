// pub fn set_panic_hook() {
//     // When the `console_error_panic_hook` feature is enabled, we can call the
//     // `set_panic_hook` function at least once during initialization, and then
//     // we will get better error messages if our code ever panics.
//     //
//     // For more details see
//     // https://github.com/rustwasm/console_error_panic_hook#readme
//     #[cfg(feature = "console_error_panic_hook")]
//     console_error_panic_hook::set_once();
// }

pub fn next(seed: &mut u64, bits: u32) -> i32 {
    *seed = (*seed * 0x5deece66d + 0xb) & ((1u64 << 48) - 1);
    ((*seed as i64) >> (48 - bits)) as i32
}

pub fn next_int(seed: &mut u64, n: i32) -> i32 {
    let mut bits;
    let mut val;
    let m: i32 = n - 1;

    if (m & n) == 0 {
        let x: u64 = n as u64 * next(seed, 31) as u64;
        return (x as i64 >> 31) as i32;
    }

    loop {
        bits = next(seed, 31);
        val = bits % n;
        if bits - val + m >= 0 {
            break;
        }
    }
    val
}

pub fn next_double(seed: &mut u64) -> f64 {
    let x = (next(seed, 26) as u64) << 27 | next(seed, 27) as u64;
    x as f64 / (1u64 << 53) as f64
}

pub fn set_seed(seed: &mut u64, value: &u64) {
    *seed = (value ^ 0x5deece66d) & ((1u64 << 48) - 1)
}

pub fn skip_next_n(seed: &mut u64, mut n: u64) {
    let mut m = 1;
    let mut a = 0;
    let mut im = 0x5deece66d_u64;
    let mut ia = 0xb_u64;

    while n != 0 {
        if n & 1 != 0 {
            m *= im;
            a = im * a + ia;
        }
        ia = (im + 1) * ia;
        im *= im;

        n >>= 1;
    }

    *seed = *seed * m + a;
    *seed &= 0xffffffffffff_u64;
}
