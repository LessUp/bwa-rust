use std::cmp::Ordering;

/// 构建后缀数组（基于倍增法，O(n log n) 排序）。
/// 输入为数值化的文本（如 0:$,1:A,2:C,3:G,4:T,5:N）。
/// 允许文本中包含多个 0 作为不同 contig 的分隔符。
pub fn build_sa(text: &[u8]) -> Vec<u32> {
    let n = text.len();
    if n == 0 {
        return Vec::new();
    }
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<i32> = text.iter().map(|&b| b as i32).collect();
    let mut tmp: Vec<i32> = vec![0; n];

    let mut k = 1usize;
    while k < n {
        sa.sort_unstable_by(|&i, &j| {
            let r1 = rank[i];
            let r2 = rank[j];
            if r1 != r2 {
                return r1.cmp(&r2);
            }
            let r1n = if i + k < n { rank[i + k] } else { -1 };
            let r2n = if j + k < n { rank[j + k] } else { -1 };
            r1n.cmp(&r2n)
        });

        tmp[sa[0]] = 0;
        for i in 1..n {
            let a = sa[i - 1];
            let b = sa[i];
            let prev = (rank[a], if a + k < n { rank[a + k] } else { -1 });
            let curr = (rank[b], if b + k < n { rank[b + k] } else { -1 });
            tmp[b] = tmp[a] + if curr != prev { 1 } else { 0 };
        }

        // 复制回 rank
        rank.copy_from_slice(&tmp);
        if rank[sa[n - 1]] as usize == n - 1 {
            break;
        }
        k <<= 1;
    }

    sa.into_iter().map(|x| x as u32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sa_basic() {
        // 文本：A C G T $  -> 1 2 3 4 0
        let text = [1u8, 2, 3, 4, 0];
        let sa = build_sa(&text);
        // 期望：后缀按字典序：$, A$, C$, G$, T$
        assert_eq!(sa, vec![4, 0, 1, 2, 3]);
    }
}
