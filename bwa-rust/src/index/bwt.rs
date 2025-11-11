/// 根据后缀数组构建 BWT。
/// text 为数值化字母表（0..SIGMA），sa 为后缀数组位置。
pub fn build_bwt(text: &[u8], sa: &[u32]) -> Vec<u8> {
    let n = text.len();
    if n == 0 {
        return Vec::new();
    }
    let mut bwt = Vec::with_capacity(n);
    for &p in sa {
        let i = p as usize;
        let prev = if i == 0 { text[n - 1] } else { text[i - 1] };
        bwt.push(prev);
    }
    bwt
}
