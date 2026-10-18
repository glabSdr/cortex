mod log_tic;
mod log_summary;
mod log_headers;
mod colorize;

pub use log_tic::*;
pub use log_summary::*;
pub use log_headers::*;


// ── ANSI codes ──────────────────────────────────────────────
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

// Marker colors: (bold_prefix, normal_prefix)
// Each marker neuron gets a unique color.
// Neurons targeting a marker inherit its color (non-bold).
struct MarkerColor {
    id: usize,
    bold: &'static str,    // marker itself: bold + color
    normal: &'static str,  // neurons targeting this marker: color only
}

// Extend this list as new marker neurons are added
const MARKER_COLORS: &[MarkerColor] = &[
    MarkerColor { id: 8,  bold: "\x1b[1;31m", normal: "\x1b[31m" },   // Threat/BLA  → Red
    MarkerColor { id: 12, bold: "\x1b[1;32m", normal: "\x1b[32m" },   // Liking      → Green
    MarkerColor { id: 15, bold: "\x1b[1;34m", normal: "\x1b[34m" },   // Antireward  → Blue
    MarkerColor { id: 16, bold: "\x1b[1;33m", normal: "\x1b[33m" },   // Own         → Yellow
    // Future markers:
    // MarkerColor { id: ??, bold: "\x1b[1;35m", normal: "\x1b[35m" }, // Novel       → Magenta
    // MarkerColor { id: ??, bold: "\x1b[1;36m", normal: "\x1b[36m" }, // Assumption  → Cyan
];

