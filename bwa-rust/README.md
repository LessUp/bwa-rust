# bwa-rust

Rust 版 BWA（灵感来源于 BWA，非一比一复刻）。

## 快速开始

```bash
# 构建
cargo build

# 运行 index 子命令（打印参考序列统计）
cargo run -- index /path/to/ref.fa -o ref
```

## 规划
- 阶段1：FASTA 读取、index 子命令、BWT/FM 索引（MVP）、精确匹配对齐
- 阶段2：FMD、SA-IS、并行与内存优化、索引序列化/加载
- 阶段3：容错与局部扩展（带状SW）、完善 SAM 输出、更多 CLI 选项
