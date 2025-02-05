use super::{profiler_counter_t, profiler_timer_t, timespec};

#[allow(clippy::derivable_impls)]
impl Default for profiler_timer_t {
    fn default() -> Self {
        Self {
            begin_timer: timespec::default(),
            time_ns: profiler_counter_t::default(),
            accumulated: 0,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for timespec {
    fn default() -> Self {
        Self {
            tv_sec: 0,
            tv_nsec: 0,
        }
    }
}

impl Default for profiler_counter_t {
    fn default() -> Self {
        Self {
            total: 0,
            samples: 0,
            min: 0,
            max: 0,
            m_oldM: 0.0,
            m_newM: 0.0,
            m_oldS: 0.0,
            m_newS: 0.0
        }
    }
}
