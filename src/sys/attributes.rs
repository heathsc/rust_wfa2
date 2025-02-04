use std::os::raw::c_int;

use super::{
    affine2p_penalties_t, affine_penalties_t, alignment_scope_t,
    alignment_scope_t_compute_alignment, alignment_scope_t_compute_score, distance_metric_t,
    distance_metric_t_edit, distance_metric_t_gap_affine, distance_metric_t_gap_affine_2p,
    distance_metric_t_gap_linear, distance_metric_t_indel, linear_penalties_t,
    wavefront_aligner_attr_default, wavefront_aligner_attr_t, wavefront_heuristic_t,
};

use crate::alignment::{AlignmentScope, DistanceMetric, MemoryMode};

impl Default for wavefront_aligner_attr_t {
    fn default() -> Self {
        unsafe { wavefront_aligner_attr_default }
    }
}

impl wavefront_aligner_attr_t {
    #[inline]
    pub fn heuristic(&mut self) -> &mut wavefront_heuristic_t {
        &mut self.heuristic
    }

    #[inline]
    pub fn set_linear_penalties(&mut self, match_: c_int, mismatch: c_int, indel: c_int) {
        self.linear_penalties.match_ = match_;
        self.linear_penalties.mismatch = mismatch;
        self.linear_penalties.indel = indel;
        self.set_distance_metric(DistanceMetric::Linear);
    }

    #[inline]
    pub fn set_affine_penalties(
        &mut self,
        match_: c_int,
        mismatch: c_int,
        gap_opening: c_int,
        gap_extension: c_int,
    ) {
        self.affine_penalties.match_ = match_;
        self.affine_penalties.mismatch = mismatch;
        self.affine_penalties.gap_opening = gap_opening;
        self.affine_penalties.gap_extension = gap_extension;
        self.set_distance_metric(DistanceMetric::Affine);
    }

    #[inline]
    pub fn set_affine2p_penalties(
        &mut self,
        match_: c_int,
        mismatch: c_int,
        gap_opening1: c_int,
        gap_extension1: c_int,
        gap_opening2: c_int,
        gap_extension2: c_int,
    ) {
        self.affine2p_penalties.match_ = match_;
        self.affine2p_penalties.mismatch = mismatch;
        self.affine2p_penalties.gap_opening1 = gap_opening1;
        self.affine2p_penalties.gap_extension1 = gap_extension1;
        self.affine2p_penalties.gap_opening2 = gap_opening2;
        self.affine2p_penalties.gap_extension2 = gap_extension2;
        self.set_distance_metric(DistanceMetric::Affine2p);
    }

    #[inline]
    pub fn linear_penalties(&self) -> &linear_penalties_t {
        &self.linear_penalties
    }

    #[inline]
    pub fn affine_penalties(&self) -> &affine_penalties_t {
        &self.affine_penalties
    }

    #[inline]
    pub fn affine2p_penalties(&self) -> &affine2p_penalties_t {
        &self.affine2p_penalties
    }

    #[inline]
    pub fn set_distance_metric(&mut self, metric: DistanceMetric) {
        self.distance_metric = metric as distance_metric_t
    }

    #[inline]
    pub fn set_alignment_scope(&mut self, scope: AlignmentScope) {
        self.alignment_scope = scope as alignment_scope_t
    }
    
    #[inline]
    pub fn set_memory_mode(&mut self, memory_mode: MemoryMode) {
        self.memory_mode = memory_mode as u32
    }
}

