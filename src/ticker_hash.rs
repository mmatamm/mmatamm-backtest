use std::{
    arch::asm,
    hash::{BuildHasher, Hasher},
};

const _CHAR_MAP: [u8; 128] = {
    let mut map = [0u8; 128];
    map[b'A' as usize] = 1;
    map[b'B' as usize] = 2;
    map[b'C' as usize] = 3;
    map[b'D' as usize] = 4;
    map[b'E' as usize] = 5;
    map[b'F' as usize] = 6;
    map[b'G' as usize] = 7;
    map[b'H' as usize] = 8;
    map[b'I' as usize] = 9;
    map[b'J' as usize] = 10;
    map[b'K' as usize] = 11;
    map[b'L' as usize] = 12;
    map[b'M' as usize] = 13;
    map[b'N' as usize] = 14;
    map[b'O' as usize] = 15;
    map[b'P' as usize] = 16;
    map[b'Q' as usize] = 17;
    map[b'R' as usize] = 18;
    map[b'S' as usize] = 19;
    map[b'T' as usize] = 20;
    map[b'U' as usize] = 21;
    map[b'V' as usize] = 22;
    map[b'W' as usize] = 23;
    map[b'X' as usize] = 24;
    map[b'Y' as usize] = 25;
    map[b'Z' as usize] = 26;
    map[b'.' as usize] = 27;
    map[b'-' as usize] = 28;
    map[b'^' as usize] = 29;
    map[b'=' as usize] = 30;
    map[b'+' as usize] = 31;
    map
};

#[inline]
pub fn ticker_hash_32(symbol: &[u8]) -> u32 {
    let mut sum: u32 = 0;

    unsafe {
        let ptr = symbol.as_ptr();
        let length = symbol.len().min(6);

        asm!(


            "        mov       {tmp_1:r}, {length:r}",
            "        xor       {sum:e}, {sum:e}",
            "        test      {tmp_1:r}, {tmp_1:r}",
            "        jbe       4f",
            "        xor       {tmp_3:e}, {tmp_3:e}",
            "        add       {ptr:r}, {tmp_1:r}",
            "2:",
            "        inc       {tmp_3:r}",
            "        mov       {tmp_4:r}, {ptr:r}",
            "        lea       {length:r}, QWORD PTR [{tmp_3:r}*8]",
            "        sub       {tmp_4:r}, {length:r}",
            "        movzx     {tmp_2:e}, BYTE PTR [7+{tmp_4:r}]",
            "        movzx     {tmp_2:e}, BYTE PTR [{char_map:r}+{tmp_2:r}]",
            "        test      {tmp_2:e}, {tmp_2:e}",
            "        je        3f",
            "        neg       {length:r}",
            "        shl       {sum:e}, 5",
            "        add       {length:r}, {tmp_1:r}",
            "        or        {sum:e}, {tmp_2:e}",
            "        mov       {tmp_2:r}, {length:r}",
            "        add       {tmp_2:r}, 7",
            "        je        4f",
            "        movzx     {tmp_2:e}, BYTE PTR [6+{tmp_4:r}]",
            "        movzx     {tmp_2:e}, BYTE PTR [{char_map:r}+{tmp_2:r}]",
            "        test      {tmp_2:e}, {tmp_2:e}",
            "        je        3f",
            "        shl       {sum:e}, 5",
            "        or        {sum:e}, {tmp_2:e}",
            "        mov       {tmp_2:r}, {length:r}",
            "        add       {tmp_2:r}, 6",
            "        je        4f",
            "        movzx     {tmp_2:e}, BYTE PTR [5+{tmp_4:r}]",
            "        movzx     {tmp_2:e}, BYTE PTR [{char_map:r}+{tmp_2:r}]",
            "        test      {tmp_2:e}, {tmp_2:e}",
            "        je        3f",
            "        shl       {sum:e}, 5",
            "        or        {sum:e}, {tmp_2:e}",
            "        mov       {tmp_2:r}, {length:r}",
            "        add       {tmp_2:r}, 5",
            "        je        4f",
            "        movzx     {tmp_2:e}, BYTE PTR [4+{tmp_4:r}]",
            "        movzx     {tmp_2:e}, BYTE PTR [{char_map:r}+{tmp_2:r}]",
            "        test      {tmp_2:e}, {tmp_2:e}",
            "        je        3f",
            "        shl       {sum:e}, 5",
            "        or        {sum:e}, {tmp_2:e}",
            "        mov       {tmp_2:r}, {length:r}",
            "        add       {tmp_2:r}, 4",
            "        je        4f",
            "        movzx     {tmp_2:e}, BYTE PTR [3+{tmp_4:r}]",
            "        movzx     {tmp_2:e}, BYTE PTR [{char_map:r}+{tmp_2:r}]",
            "        test      {tmp_2:e}, {tmp_2:e}",
            "        je        3f",
            "        shl       {sum:e}, 5",
            "        or        {sum:e}, {tmp_2:e}",
            "        mov       {tmp_2:r}, {length:r}",
            "        add       {tmp_2:r}, 3",
            "        je        4f",
            "        movzx     {tmp_2:e}, BYTE PTR [2+{tmp_4:r}]",
            "        movzx     {tmp_2:e}, BYTE PTR [{char_map:r}+{tmp_2:r}]",
            "        test      {tmp_2:e}, {tmp_2:e}",
            "        je        3f",
            "        shl       {sum:e}, 5",
            "        or        {sum:e}, {tmp_2:e}",
            "        mov       {tmp_2:r}, {length:r}",
            "        add       {tmp_2:r}, 2",
            "        je        4f",
            "        movzx     {tmp_2:e}, BYTE PTR [1+{tmp_4:r}]",
            "        movzx     {tmp_2:e}, BYTE PTR [{char_map:r}+{tmp_2:r}]",
            "        test      {tmp_2:e}, {tmp_2:e}",
            "        je        3f",
            "        shl       {sum:e}, 5",
            "        or        {sum:e}, {tmp_2:e}",
            "        mov       {tmp_2:r}, {length:r}",
            "        inc       {tmp_2:r}",
            "        je        4f",
            "        movzx     {tmp_4:e}, BYTE PTR [{tmp_4:r}]",
            "        movzx     {tmp_4:e}, BYTE PTR [{char_map:r}+{tmp_4:r}]",
            "        test      {tmp_4:e}, {tmp_4:e}",
            "        je        3f",
            "        shl       {sum:e}, 5",
            "        or        {sum:e}, {tmp_4:e}",
            "        test      {length:r}, {length:r}",
            "        jne       2b",
            "3:",
            "        xor       {sum:e}, {sum:e}",
            "4:",

            ptr = in(reg) ptr,
            length = in(reg) length,
            sum = inout(reg) sum,
            tmp_1 = out(reg) _,
            tmp_2 = out(reg) _,
            tmp_3 = out(reg) _,
            tmp_4 = out(reg) _,
            char_map = in(reg) _CHAR_MAP.as_ptr(),
            options(nostack, preserves_flags)
        )
    }

    sum
}

#[inline]
pub(crate) fn ticker_hash_64(symbol: &[u8]) -> u64 {
    let mut buf = [0u8; 8];
    let len = symbol.len().min(8);
    buf[..len].copy_from_slice(&symbol[..len]);
    u64::from_le_bytes(buf)
}

pub(crate) struct TickerHasher {
    result: u64,
}

impl Default for TickerHasher {
    fn default() -> Self {
        TickerHasher { result: 0 }
    }
}

impl Hasher for TickerHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.result
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.result = ticker_hash_64(bytes)
    }
}

#[derive(Clone)]
pub(crate) struct TickerHasherBuilder;

impl BuildHasher for TickerHasherBuilder {
    type Hasher = TickerHasher;

    fn build_hasher(&self) -> Self::Hasher {
        TickerHasher::default()
    }
}

impl Default for TickerHasherBuilder {
    fn default() -> Self {
        TickerHasherBuilder {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut hasher = TickerHasher::default();
        hasher.write(b"");
        assert_eq!(hasher.finish(), 0);
    }

    #[test]
    fn test_two_bytes() {
        let mut hasher = TickerHasher::default();
        hasher.write(b"AB");
        // Little-endian: 65 + (66 << 8) = 65 + 16896 = 16961
        assert_eq!(hasher.finish(), 16961);
    }

    #[test]
    fn test_eight_bytes() {
        let mut hasher = TickerHasher::default();
        hasher.write(b"ABCDEFGH");
        // Little-endian: lowest byte is 'A', highest is 'H'
        let expected = u64::from_le_bytes(*b"ABCDEFGH");
        assert_eq!(hasher.finish(), expected);
    }

    #[test]
    fn test_more_than_eight_bytes() {
        let mut hasher = TickerHasher::default();
        hasher.write(b"ABCDEFGHIJKLMN");
        // Only first 8 bytes are used
        let expected = u64::from_le_bytes(*b"ABCDEFGH");
        assert_eq!(hasher.finish(), expected);
    }
}
