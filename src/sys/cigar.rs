use std::{
    fmt,
    os::raw::{c_char, c_int},
};

use super::{
    affine2p_penalties_t, affine_penalties_t, cigar_print_pretty, cigar_score_edit,
    cigar_score_gap_affine, cigar_score_gap_affine2p, cigar_score_gap_linear, cigar_sprint,
    cigar_sprint_SAM_CIGAR, cigar_t, linear_penalties_t, _IO_FILE,
};

impl cigar_t {
    #[inline]
    pub fn score(&self) -> c_int {
        self.score
    }

    #[inline]
    pub fn begin_offset(&self) -> usize {
        self.begin_offset as usize
    }

    #[inline]
    pub fn end_offset(&self) -> usize {
        self.end_offset as usize
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.begin_offset >= self.end_offset
    }

    #[inline]
    pub fn operations(&self) -> &[u8] {
        let b = self.begin_offset();
        let l = self.end_offset().saturating_sub(b);
        let p = self.operations;
        assert!(!p.is_null(), "Cigar operations is Null");
        unsafe { std::slice::from_raw_parts(p.add(b) as *const u8, l) }
    }
    
    #[inline]
    pub fn score_edit(&self) -> c_int {
        unsafe { cigar_score_edit(self) }
    }

    #[inline]
    pub fn score_gap_linear(&self, penalties: &linear_penalties_t) -> c_int {
        unsafe { cigar_score_gap_linear(self, penalties) }
    }

    #[inline]
    pub fn score_gap_affine(&self, penalties: &affine_penalties_t) -> c_int {
        unsafe { cigar_score_gap_affine(self, penalties) }
    }

    #[inline]
    pub fn score_gap_affine2p(&self, penalties: &affine2p_penalties_t) -> c_int {
        unsafe { cigar_score_gap_affine2p(self, penalties) }
    }

    pub fn sam_cigar(&mut self, show_mismatches: bool) -> String {
        if !self.is_null() {
            let buffer_length = 4 * (self.end_offset() - self.begin_offset()) + 20;
            let mut buf: Vec<u8> = vec![0; buffer_length];
            let l = unsafe {
                cigar_sprint_SAM_CIGAR(buf.as_mut_ptr() as *mut c_char, self, show_mismatches)
                    as usize
            };
            buf.resize(l, 0);
            String::from_utf8(buf).expect("cigar_sprint_SAM_CIGAR returned not utf8 bytes")
        } else {
            String::new()
        }
    }
}

impl fmt::Display for cigar_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_null() {
            let buffer_length = 2 * (self.end_offset() - self.begin_offset()) + 20;
            let mut buf: Vec<u8> = vec![0; buffer_length];
            let l = unsafe {
                cigar_sprint(buf.as_mut_ptr() as *mut c_char, self, !f.alternate()) as usize
            };
            buf.resize(l, 0);
            write!(
                f,
                "{}",
                std::str::from_utf8(&buf).expect("cigar_sprint() returned non utf8 bytes")
            )
        } else {
            Ok(())
        }
    }
}
