use super::wavefront_plot_attr_t;

impl Default for wavefront_plot_attr_t {
    fn default() -> Self {
        Self {
            enabled: false,
            resolution_points: 2000,
            align_level: 0,
        }
    }
}