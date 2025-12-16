use chrono::Timelike;

pub struct LichtZeitPegel {
    bits: u64,
}

impl LichtZeitPegel {
    fn new() -> Self {
        Self { bits: 0 }
    }

    fn set(&mut self, index: u8, value: bool) {
        assert!(index < 50);
        let mask = 1u64 << index;
        if value {
            self.bits |= mask;
        } else {
            self.bits &= !mask;
        }
    }

    fn get(&self, index: u8) -> bool {
        assert!(index < 50);
        (self.bits >> index) & 1 == 1
    }

    // convert timestamp to unary representation
    fn from_timestamp(time: chrono::NaiveTime) -> Self {
        let mut lzp = Self::new();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();

        let hour_upper = hour / 10;
        let hour_lower = hour % 10;

        let minute_upper = minute / 10;
        let minute_lower = minute % 10;

        let second_upper = second / 10;
        let second_lower = second % 10;

        let hour_bits = Self::to_bits(hour_upper as u32, hour_lower as u32);
        let minute_bits = Self::to_bits(minute_upper as u32, minute_lower as u32);
        let second_bits = Self::to_bits(second_upper as u32, second_lower as u32);

        println!(
            "{:b}",
            (hour_bits << 40) | (minute_bits << 20) | second_bits
        );
        lzp.bits = (hour_bits << 40) | (minute_bits << 20) | second_bits;

        lzp
    }

    // map 1, 3 to
    // 1000000000, 1110000000
    fn to_bits(upper: u32, lower: u32) -> u64 {
        let upper = upper.min(10);
        let lower = lower.min(19);

        // 10-bit field, left-aligned
        let upper_bits = ((1u32 << upper) - 1) << (20 - upper);

        // 10-bit field, left-aligned
        let lower_bits = ((1u32 << lower) - 1) << (10 - lower);

        upper_bits as u64 | lower_bits as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that 13:37:00 is represented correctly
    fn displays_13_37() {
        let hour_upper = "1000000000";
        let hour_lower = "1110000000";

        let minute_upper = "1110000000";
        let minute_lower = "1111111000";

        let second_upper = "0000000000";
        let second_lower = "0000000000";

        let expected = format!(
            "{}{}{}{}{}{}",
            hour_upper, hour_lower, minute_upper, minute_lower, second_upper, second_lower
        );

        let time = chrono::NaiveTime::from_hms_opt(13, 37, 0).unwrap();
        let lzp = LichtZeitPegel::from_timestamp(time);

        let result = format!("{:b}", lzp.bits);

        assert_eq!(result, expected);
    }

    #[test]
    fn to_bits() {
        let exptected = "10000000001110000000";
        let bits = LichtZeitPegel::to_bits(1, 3);

        let mut result = String::new();
        for i in (0..20).rev() {
            if (bits >> i) & 1 == 1 {
                result.push('1');
            } else {
                result.push('0');
            }
        }
        assert_eq!(result, exptected);
    }
}
