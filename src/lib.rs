const ZEROED: [u8; 32] = [0u8; 32];

#[inline(always)]
pub fn is_zero_terminated(hash: &[u8], count: usize) -> bool {
    assert_eq!(hash.len(), 32);
    assert!(0 < count && count <= 64);

    let count_bytes = count / 2;
    if hash[32 - count_bytes..] == ZEROED[..count_bytes] {
        !(count % 2 == 1 && hash[32 - 1 - count_bytes] % 16 != 0)
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn incorrect_hash_length() {
        is_zero_terminated(&[1], 1);
    }

    #[test]
    #[should_panic]
    fn incorrect_count_is_less_zero() {
        is_zero_terminated(ZEROED.as_slice(), 0);
    }

    #[test]
    #[should_panic]
    fn incorrect_count_is_great_than_64() {
        is_zero_terminated(ZEROED.as_slice(), 65);
    }

    #[test]
    fn zero() {
        for i in 1..=64 {
            assert!(is_zero_terminated(ZEROED.as_slice(), i));
        }
    }

    #[test]
    fn no_tail_zero() {
        let v = &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 21,
        ];
        for i in 1..=64 {
            assert!(!is_zero_terminated(v.as_slice(), i));
        }
    }

    #[test]
    fn single_tail_zero() {
        let v = &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 48,
        ];
        assert!(is_zero_terminated(v.as_slice(), 1));
        for i in 2..=64 {
            assert!(!is_zero_terminated(v.as_slice(), i));
        }
    }

    #[test]
    fn even_tail_zeroes() {
        let v = &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 0u8,
        ];
        assert!(is_zero_terminated(v.as_slice(), 1));
        assert!(is_zero_terminated(v.as_slice(), 2));
        for i in 3..=64 {
            assert!(!is_zero_terminated(v.as_slice(), i));
        }
    }

    #[test]
    fn odd_tail_zeroes() {
        let v = &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            2, 16, 0u8,
        ];
        assert!(is_zero_terminated(v.as_slice(), 1));
        assert!(is_zero_terminated(v.as_slice(), 2));
        assert!(is_zero_terminated(v.as_slice(), 3));
        for i in 4..=64 {
            assert!(!is_zero_terminated(v.as_slice(), i));
        }
    }
}
