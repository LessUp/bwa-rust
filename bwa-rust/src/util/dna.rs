pub const SIGMA: usize = 6; // {0:$, 1:A, 2:C, 3:G, 4:T, 5:N}

#[inline]
pub fn to_alphabet(b: u8) -> u8 {
    if b == 0 { return 0; }
    match b.to_ascii_uppercase() {
        b'A' => 1,
        b'C' => 2,
        b'G' => 3,
        b'T' | b'U' => 4,
        b'N' => 5,
        _ => 5, // map others to N
    }
}

#[inline]
pub fn from_alphabet(a: u8) -> u8 {
    match a {
        0 => 0,
        1 => b'A',
        2 => b'C',
        3 => b'G',
        4 => b'T',
        5 => b'N',
        _ => b'N',
    }
}

pub fn normalize_seq(seq: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(seq.len());
    for &b in seq {
        let up = b.to_ascii_uppercase();
        let nb = match up {
            b'A' | b'C' | b'G' | b'T' | b'N' => up,
            b'U' => b'T',
            _ => b'N',
        };
        out.push(nb);
    }
    out
}

#[inline]
pub fn complement(base: u8) -> u8 {
    match base.to_ascii_uppercase() {
        b'A' => b'T',
        b'C' => b'G',
        b'G' => b'C',
        b'T' | b'U' => b'A',
        _ => b'N',
    }
}

pub fn revcomp(seq: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(seq.len());
    for &b in seq.iter().rev() {
        out.push(complement(b));
    }
    out
}
