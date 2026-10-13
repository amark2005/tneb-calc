
#![no_std]

use core::ffi::c_char;
use core::ptr;

const ERROR: i32 = 1;
const SUCCESS: i32 = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_stod(
    input: *const c_char,
    output: *mut f64,
) -> i32 {
    if input.is_null() || output.is_null() {
        return ERROR;
    }

    let mut i: usize = 0;

    // SAFETY: Caller must provide a valid NUL-terminated C string.
    unsafe {
        loop {
            let c = *input.add(i) as u8;
            if !matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
                break;
            }
            i += 1;
        }
    }

    let mut negative = false;

    unsafe {
        match *input.add(i) as u8 {
            b'-' => {
                negative = true;
                i += 1;
            }
            b'+' => i += 1,
            _ => {}
        }
    }

    let mut mantissa = 0.0_f64;
    let mut digits = 0_u32;
    let mut fractional_digits = 0_i32;
    let mut after_dot = false;

    loop {
        let c = unsafe { *input.add(i) as u8 };

        if c.is_ascii_digit() {
            let digit = (c - b'0') as f64;
            mantissa = mantissa * 10.0 + digit;

            if mantissa > f64::MAX {
                return ERROR;
            }

            digits = digits.saturating_add(1);

            if after_dot {
                fractional_digits += 1;

                if fractional_digits > 10000 {
                    return ERROR;
                }
            }

            i += 1;
        } else if c == b'.' && !after_dot {
            after_dot = true;
            i += 1;
        } else {
            break;
        }
    }

    if digits == 0 {
        return ERROR;
    }

    let mut exponent = 0_i32;

    if matches!(unsafe { *input.add(i) as u8 }, b'e' | b'E') {
        let _exponent_start = i;
        i += 1;

        let mut exponent_negative = false;

        match unsafe { *input.add(i) as u8 } {
            b'-' => {
                exponent_negative = true;
                i += 1;
            }
            b'+' => i += 1,
            _ => {}
        }

        let mut exponent_digits = 0_u32;
        let mut parsed_exponent = 0_i32;

        loop {
            let c = unsafe { *input.add(i) as u8 };

            if !c.is_ascii_digit() {
                break;
            }

            exponent_digits += 1;
            parsed_exponent = parsed_exponent
                .saturating_mul(10)
                .saturating_add((c - b'0') as i32)
                .min(10000);

            i += 1;
        }

        if exponent_digits != 0 {
            exponent = if exponent_negative {
                -parsed_exponent
            } else {
                parsed_exponent
            };
        }
    }

    let scale = exponent.saturating_sub(fractional_digits);

    if scale > 308 || scale < -400 {
        return ERROR;
    }

    let mut value = mantissa;

    if scale > 0 {
        let mut n = scale;
        while n > 0 {
            value *= 10.0;

            if value > f64::MAX {
                return ERROR;
            }

            n -= 1;
        }
    } else {
        let mut n = scale;
        while n < 0 {
            value /= 10.0;
            n += 1;
        }
    }

    if value == 0.0 && mantissa != 0.0 {
        return ERROR;
    }

    if negative {
        value = -value;
    }

    // SAFETY: Caller must provide a valid writable f64 pointer.
    unsafe {
        ptr::write(output, value);
    }

    SUCCESS
}
