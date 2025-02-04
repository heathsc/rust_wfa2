use std::os::raw::c_int;

use super::{
    wavefront_heuristic_clear,
    wavefront_heuristic_set_banded_adaptive, wavefront_heuristic_set_banded_static,
    wavefront_heuristic_set_none, wavefront_heuristic_set_wfadaptive,
    wavefront_heuristic_set_wfmash, wavefront_heuristic_set_xdrop, wavefront_heuristic_set_zdrop,
    wavefront_heuristic_t,
};

impl wavefront_heuristic_t {
    #[inline]
    pub fn set_none(&mut self) {
        unsafe { wavefront_heuristic_set_none(self) }
    }

    #[inline]
    pub fn set_wfadaptive(
        &mut self,
        min_wavefront_length: c_int,
        max_distance_threshold: c_int,
        steps_between_cutoffs: c_int,
    ) {
        unsafe {
            wavefront_heuristic_set_wfadaptive(
                self,
                min_wavefront_length,
                max_distance_threshold,
                steps_between_cutoffs,
            )
        }
    }

    #[inline]
    pub fn set_wfmash(
        &mut self,
        min_wavefront_length: c_int,
        max_distance_threshold: c_int,
        steps_between_cutoffs: c_int,
    ) {
        unsafe {
            wavefront_heuristic_set_wfmash(
                self,
                min_wavefront_length,
                max_distance_threshold,
                steps_between_cutoffs,
            )
        }
    }

    #[inline]
    pub fn set_xdrop(&mut self, x_drop: c_int, steps_between_cutoffs: c_int) {
        unsafe { wavefront_heuristic_set_xdrop(self, x_drop, steps_between_cutoffs) }
    }

    #[inline]
    pub fn set_zdrop(&mut self, z_drop: c_int, steps_between_cutoffs: c_int) {
        unsafe { wavefront_heuristic_set_zdrop(self, z_drop, steps_between_cutoffs) }
    }

    #[inline]
    pub fn set_banded_static(&mut self, band_min_k: c_int, band_max_k: c_int) {
        unsafe { wavefront_heuristic_set_banded_static(self, band_min_k, band_max_k) }
    }

    #[inline]
    pub fn set_banded_adaptive(
        &mut self,
        band_min_k: c_int,
        band_max_k: c_int,
        steps_between_cutoffs: c_int,
    ) {
        unsafe {
            wavefront_heuristic_set_banded_adaptive(
                self,
                band_min_k,
                band_max_k,
                steps_between_cutoffs,
            )
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        unsafe { wavefront_heuristic_clear(self) }
    }
}
