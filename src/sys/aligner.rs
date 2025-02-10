use std::os::raw::c_int;

use super::{
    cigar_t, wavefront_align, wavefront_align_status_t, wavefront_aligner_set_alignment_end_to_end, wavefront_aligner_set_alignment_extension, wavefront_aligner_set_alignment_free_ends, wavefront_aligner_t, wavefront_pos_t
};
use crate::error::*;

impl wavefront_aligner_t {
    #[inline]
    pub fn align(&mut self, pattern: &[u8], text: &[u8]) -> Result<WfaStatus, WfaError> {
        let u8_to_i8 = |s: &[u8]| (s as *const [u8] as *const i8);

        check_status(unsafe {
            wavefront_align(
                self,
                u8_to_i8(pattern),
                pattern.len() as c_int,
                u8_to_i8(text),
                text.len() as c_int,
            )
        })
    }

    #[inline]
    pub fn align_str(&mut self, pattern: &str, text: &str) -> Result<WfaStatus, WfaError> {
        self.align(pattern.as_bytes(), text.as_bytes())
    }
    
    #[inline]
    pub fn end_pos(&self) -> &wavefront_pos_t {
        &self.alignment_end_pos
    }
    
    #[inline]
    pub fn cigar(&self) -> &cigar_t {
        let p = self.cigar;
        assert!(!p.is_null());
        unsafe { &*p }
    }

    #[inline]
    pub fn cigar_mut(&mut self) -> &mut cigar_t {
        let p = self.cigar;
        assert!(!p.is_null());
        unsafe { &mut *p }
    }
    
    #[inline]
    pub fn set_alignment_end_to_end(&mut self) {
        unsafe { wavefront_aligner_set_alignment_end_to_end(self) }
    }
    
    #[inline]
    pub fn set_alignment_free_ends(&mut self, pattern_begin_free: c_int, pattern_end_free: c_int, text_begin_free: c_int, text_end_free: c_int) {
        unsafe { wavefront_aligner_set_alignment_free_ends(self, pattern_begin_free, pattern_end_free, text_begin_free, text_end_free) }
    }
    
    #[inline]
    pub fn set_alignment_extension(&mut self) {
        unsafe { wavefront_aligner_set_alignment_extension(self) }
    }
    
    pub fn status(&self) -> &wavefront_align_status_t {
        &self.align_status
    }
}

impl wavefront_pos_t {
    #[inline] 
    pub fn offsets(&self) -> (c_int, c_int) {
        (self.offset - self.k, self.offset)
    }
    
    #[inline]
    pub fn score(&self) -> c_int {
        self.score
    }
}

impl wavefront_align_status_t {
    #[inline]
    pub fn score(&self) -> c_int {
        self.score
    }
    
    #[inline]
    pub fn status(&self) -> c_int {
        self.status
    }
    
    #[inline]
    pub fn num_null_steps(&self) -> c_int {
        self.num_null_steps
    }
    
    #[inline]
    pub fn memory_used(&self) -> u64 {
        self.memory_used
    }
    
    #[inline]
    pub fn dropped(&self) -> bool {
        self.dropped
    }
}