use anyhow::Result;
use std::io::Write;

use crate::index::fm::FMIndex;
use crate::io::fastq::FastqReader;
use crate::util::dna;

pub fn align_fastq(index_path: &str, fastq_path: &str, out_path: Option<&str>) -> Result<()> {
    // load FM index
    let fm = FMIndex::load_from_file(index_path)?;

    // open FASTQ
    let fq = std::fs::File::open(fastq_path)?;
    let mut reader = FastqReader::new(std::io::BufReader::new(fq));

    // writer
    let mut out_box: Box<dyn Write> = if let Some(p) = out_path {
        Box::new(std::io::BufWriter::new(std::fs::File::create(p)?))
    } else {
        Box::new(std::io::BufWriter::new(std::io::stdout()))
    };

    // SAM header (minimal)
    for c in &fm.contigs {
        writeln!(out_box, "@SQ\tSN:{}\tLN:{}", c.name, c.len)?;
    }

    // iterate reads
    while let Some(rec) = reader.next_record()? {
        let qname = &rec.id;
        let seq = &rec.seq;
        let qual = &rec.qual;

        // prepare forward
        let fwd_norm = dna::normalize_seq(seq);
        let fwd_alpha: Vec<u8> = fwd_norm.iter().map(|&b| dna::to_alphabet(b)).collect();
        // prepare reverse complement
        let rev = dna::revcomp(seq);
        let rev_alpha: Vec<u8> = rev.iter().map(|&b| dna::to_alphabet(b)).collect();

        let mut write_unmapped = true;

        // try forward
        if let Some((l, r)) = fm.backward_search(&fwd_alpha) {
            if r > l {
                let pos = fm.sa_interval_positions(l, r)[0];
                if let Some((ci, off)) = fm.map_text_pos(pos) {
                    let contig = &fm.contigs[ci];
                    // FLAG 0: forward strand
                    let flag = 0;
                    let rname = &contig.name;
                    let pos1 = off + 1; // 1-based
                    let mapq = 255;
                    let cigar = format!("{}M", seq.len());
                    writeln!(
                        out_box,
                        "{}\t{}\t{}\t{}\t{}\t{}\t*\t0\t0\t{}\t{}",
                        qname,
                        flag,
                        rname,
                        pos1,
                        mapq,
                        cigar,
                        String::from_utf8_lossy(seq),
                        String::from_utf8_lossy(qual),
                    )?;
                    write_unmapped = false;
                }
            }
        }

        // try reverse if forward failed
        if write_unmapped {
            if let Some((l, r)) = fm.backward_search(&rev_alpha) {
                if r > l {
                    let pos = fm.sa_interval_positions(l, r)[0];
                    if let Some((ci, off)) = fm.map_text_pos(pos) {
                        let contig = &fm.contigs[ci];
                        // FLAG 16: reverse complemented
                        let flag = 16;
                        let rname = &contig.name;
                        let pos1 = off + 1; // 1-based
                        let mapq = 255;
                        let cigar = format!("{}M", seq.len());
                        writeln!(
                            out_box,
                            "{}\t{}\t{}\t{}\t{}\t{}\t*\t0\t0\t{}\t{}",
                            qname,
                            flag,
                            rname,
                            pos1,
                            mapq,
                            cigar,
                            String::from_utf8_lossy(seq),
                            String::from_utf8_lossy(qual),
                        )?;
                        write_unmapped = false;
                    }
                }
            }
        }

        if write_unmapped {
            // unmapped: FLAG 4, RNEXT/PNEXT/SEQ/QUAL as per SAM minimal
            let flag = 4;
            writeln!(
                out_box,
                "{}\t{}\t*\t0\t0\t*\t*\t0\t0\t{}\t{}",
                qname,
                flag,
                String::from_utf8_lossy(seq),
                String::from_utf8_lossy(qual),
            )?;
        }
    }

    Ok(())
}
