#[macro_use]
extern crate approx;

fn round_up(len: i32, base: i32) -> i32 {
    ((len + base) / base) * base
}

fn round_down(len: i32, base: i32) -> i32 {
    (len / base) * base
}
// all based on 4k values
pub fn calculate_waf(offset: i32, len: i32, iu: i32) -> f64 {
    let mut io_len = len;
    let mut extra_io: i32 = 0;

    if (offset & (iu - 1)) != 0 {
        extra_io += iu;
        io_len -= (round_up(offset, iu)) - offset;
    }

    if io_len < 0 {
        return extra_io as f64 / len as f64;
    }

    // End is not aligned
    if ((offset + len) & (iu - 1)) != 0 {
        extra_io += iu;
        io_len -= offset + len - (round_down(offset + len, iu));
    }

    (io_len as f64 + extra_io as f64) / len as f64
}

#[cfg(test)]
mod tests {
    use crate::calculate_waf;

    #[test]
    fn waf_stress() {
        let result = 2.0;
        // offset 4k, length 8k
        assert_eq!(calculate_waf(1, 2, 4), result);
        let result = 4.0;
        // offset 12k, length 8k
        assert_eq!(calculate_waf(3, 2, 4), result);
        let result = 1.33;
        // offset 4k, length 12k
        assert_relative_eq!(calculate_waf(1, 3, 4), result, max_relative = 0.01);
        let result = 2.66;
        // offset 8k, length 12k
        assert_relative_eq!(calculate_waf(2, 3, 4), result, max_relative = 0.01);
        let result = 1.03125;
        // offset 8k, length 512k
        assert_relative_eq!(calculate_waf(2, 128, 4), result, max_relative = 0.000001);
        let result = 1.2;
        // offset 8k, length 40k
        assert_relative_eq!(calculate_waf(2, 10, 4), result, max_relative = 0.000001);
        let result = 1.5;
        // offset 24k, length 32k
        assert_relative_eq!(calculate_waf(6, 8, 4), result, max_relative = 0.000001);
        let result = 1.0625;
        // offset 12k, length 256k
        assert_relative_eq!(calculate_waf(3, 64, 4), result, max_relative = 0.000001);
    }
}
