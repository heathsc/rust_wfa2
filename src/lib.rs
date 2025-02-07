pub mod aligner;
pub mod error;
pub mod alignment;

pub(crate) mod sys;

mod test {
    #[allow(unused_imports)]
    use super::{
        alignment::{Attributes, AlignmentScope},
        aligner::WfaAligner,
        error::{WfaError, WfaStatus},
    };

    #[test]
    fn simple_align() {
        let pattern = "TCTTTACTCGCGCGTTGGAGAAATACAATAGT";
        let text = "TCTATACTGCGCGTTTGGAGAAATAAAATAGT";

        let mut attributes = Attributes::default();
        attributes.heuristic().set_none();
        attributes.set_affine_penalties(0, 4, 6, 2);
        attributes.set_alignment_scope(AlignmentScope::Alignment);

        let mut aligner = WfaAligner::new(&attributes);

        match aligner.align_str(pattern, text) {
            Ok(s) => println!("OK: {s}"),
            Err(s) => println!("Error: {s}"),
        }

        println!("WFA aligner returns score: {}", aligner.cigar().score());
        println!("  PATTERN: {pattern}");
        println!("  TEXT:    {text}");
        println!(
            "  SCORE (RE)COMPUTED {}",
            aligner
                .cigar()
                .score_gap_affine(attributes.affine_penalties())
        );
        assert_eq!(aligner.cigar().score(), -24);
    }

    #[test]
    fn adaptives_align() {
        let pattern = "TCTTTACTCGCGCGTTGGAGAAATACAATAGT";
        let text = "TCTATACTGCGCGTTTGGAGAAATAAAATAGT";

        let mut attributes = Attributes::default();
        attributes.set_affine_penalties(0, 4, 6, 2);
        attributes.heuristic().set_wfadaptive(10, 50, 1);

        let mut aligner = WfaAligner::new(&attributes);

        match aligner.align_str(pattern, text) {
            Ok(s) => println!("OK: {s}"),
            Err(s) => println!("Error: {s}"),
        }

        println!("WFA aligner returns score: {} {}", aligner.cigar().score(), aligner.cigar());
        assert_eq!(aligner.cigar().score(), -24);
        
        let (misms, ins, del) =
            aligner
                .cigar()
                .operations()
                .iter()
                .fold((0, 0, 0), |(m, i, d), c| match c {
                    b'M' | 0 => (m, i, d),
                    b'I' => (m, i + 1, d),
                    b'D' => (m, i, d + 1),
                    b'X' => (m + 1, i, d),
                    _ => panic!("Unknown operation {c}"),
                });
        
        println!("Alignment contains {misms} mismatches, {ins} insertions, and {del} deletions");

        assert_eq!((misms, ins, del), (2,1,1));
    }
    
    #[test]
    fn free_ends() {
        let pattern = "TCTTTACTCGCGCGTTGGAGAAATACAATAGT";
        let text = "AACTGATCGTTATCTATACTGCGCGTTTGGAGAAATAAAATAGTGGTAATCTATCCCATGGTAC";

        let mut attributes = Attributes::default();
        attributes.heuristic().set_none();
        attributes.set_affine_penalties(0, 4, 6, 2);
        attributes.set_alignment_scope(AlignmentScope::Alignment);
        attributes.set_memory_mode(crate::alignment::MemoryMode::Low);
        let mut aligner = WfaAligner::new(&attributes);

        aligner.set_alignment_free_ends(0, 0, 30, 30);
        match aligner.align_str(pattern, text) {
            Ok(s) => println!("OK: {s}"),
            Err(s) => println!("Error: {s}"),
        }

        println!("WFA aligner returns score: {}", aligner.cigar().score());
        println!("  PATTERN: {pattern}");
        println!("  TEXT:    {text}");
        println!(
            "  SCORE (RE)COMPUTED {}",
            aligner
                .cigar()
                .score_gap_affine(attributes.affine_penalties())
        );

        assert_eq!(aligner.cigar().score(), -24);
    }
}
