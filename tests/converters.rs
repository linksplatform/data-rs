use platform_data::{AddrToRaw, RawToAddr};

#[test]
fn test_addr_to_raw_roundtrip_u8() {
    for i in 0..=u8::MAX {
        let raw = AddrToRaw.convert(i);
        let addr = RawToAddr.convert(raw);
        assert_eq!(addr, i, "roundtrip failed for u8 value {i}");
    }
}

#[test]
fn test_addr_to_raw_roundtrip_u16() {
    for i in (0..=u16::MAX).step_by(251) {
        let raw = AddrToRaw.convert(i);
        let addr = RawToAddr.convert(raw);
        assert_eq!(addr, i);
    }
}

#[test]
fn test_addr_to_raw_roundtrip_u32() {
    let values: Vec<u32> = vec![0, 1, 100, 1000, u32::MAX / 2, u32::MAX - 1, u32::MAX];
    for v in values {
        let raw = AddrToRaw.convert(v);
        let addr = RawToAddr.convert(raw);
        assert_eq!(addr, v);
    }
}

#[test]
fn test_addr_to_raw_roundtrip_u64() {
    let values: Vec<u64> = vec![0, 1, 100, u64::MAX / 2, u64::MAX - 1, u64::MAX];
    for v in values {
        let raw = AddrToRaw.convert(v);
        let addr = RawToAddr.convert(raw);
        assert_eq!(addr, v);
    }
}

#[test]
fn test_addr_to_raw_roundtrip_usize() {
    let values: Vec<usize> = vec![0, 1, 42, usize::MAX / 2, usize::MAX];
    for v in values {
        let raw = AddrToRaw.convert(v);
        let addr = RawToAddr.convert(raw);
        assert_eq!(addr, v);
    }
}

#[test]
fn test_addr_to_raw_zero() {
    let raw = AddrToRaw.convert(0u64);
    let addr = RawToAddr.convert(raw);
    assert_eq!(addr, 0);
}

#[test]
fn test_raw_to_addr_zero() {
    let addr = RawToAddr.convert(0u64);
    let raw = AddrToRaw.convert(addr);
    assert_eq!(raw, 0);
}
