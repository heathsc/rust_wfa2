use super::sys::{
    alignment_scope_t_compute_alignment, alignment_scope_t_compute_score, distance_metric_t_edit,
    distance_metric_t_gap_affine, distance_metric_t_gap_affine_2p, distance_metric_t_gap_linear,
    distance_metric_t_indel, wavefront_aligner_attr_t, wavefront_memory_t_wavefront_memory_high,
    wavefront_memory_t_wavefront_memory_low, wavefront_memory_t_wavefront_memory_med,
    wavefront_memory_t_wavefront_memory_ultralow,
};

pub type Attributes = wavefront_aligner_attr_t;

#[allow(clippy::unnecessary_cast)]
#[repr(u32)]
pub enum DistanceMetric {
    Edit = distance_metric_t_edit as u32,
    Indel = distance_metric_t_indel as u32,
    Linear = distance_metric_t_gap_linear as u32,
    Affine = distance_metric_t_gap_affine as u32,
    Affine2p = distance_metric_t_gap_affine_2p as u32,
}

#[allow(clippy::unnecessary_cast)]
#[repr(u32)]
pub enum AlignmentScope {
    Score = alignment_scope_t_compute_score as u32,
    Alignment = alignment_scope_t_compute_alignment as u32,
}

#[allow(clippy::unnecessary_cast)]
#[repr(u32)]
pub enum MemoryMode {
    High = wavefront_memory_t_wavefront_memory_high as u32,
    Medium = wavefront_memory_t_wavefront_memory_med as u32,
    Low = wavefront_memory_t_wavefront_memory_low as u32,
    UltraLow = wavefront_memory_t_wavefront_memory_ultralow as u32,
}
