use std::os::raw::c_int;

use super::{
    alignment_form_t, alignment_span_t_alignment_end2end, alignment_system_t,
    distance_metric_t_gap_affine, profiler_timer_t,
};

impl Default for alignment_form_t {
    fn default() -> Self {
        Self {
            span: alignment_span_t_alignment_end2end,
            extension: false,
            pattern_begin_free: 0,
            pattern_end_free: 0,
            text_begin_free: 0,
            text_end_free: 0,
        }
    }
}

impl Default for alignment_system_t {
    fn default() -> Self {
        Self {
            max_alignment_steps: c_int::MAX,
            probe_interval_global: 3000,
            probe_interval_compact: 6000,
            max_memory_compact: u64::MAX,
            max_memory_resident: u64::MAX,
            max_memory_abort: u64::MAX,
            verbose: 0,
            check_alignment_correct: false,
            max_num_threads: 1,
            min_offsets_per_thread: 500,
            max_partial_compacts: 0,
            timer: profiler_timer_t::default(),
        }
    }
}
