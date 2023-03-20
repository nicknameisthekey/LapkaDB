pub fn to_u16(slice: &[u8]) -> u16 {
    assert_eq!(2, slice.len());
    ((slice[0] as u16) << 8) | slice[1] as u16
}

