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

// Marker colors
struct MarkerColor {
    id: usize,
    bold: &'static str,
    normal: &'static str,
}

// ── Neuron ID constants (keep in sync with gen_mesh.py) ──
const VTA_INPUT: usize = 1;
const VTA_OUTPUT: usize = 2;
const STOP_VTA_GABA_OUT: usize = 9;   // gaba output of STOP_VTA ring
const ENKEPHALIN_TO_VTA: usize = 10;
const LC_OUTPUT: usize = 19;
const STOP_STRESS: usize = 31;
const DRN_OUTPUT: usize = 33;
const FLASH_YES: usize = 56;
const FLASH: usize = 59;

const MARKER_COLORS: &[MarkerColor] = &[
    MarkerColor { id: 61, bold: "\x1b[1;31m", normal: "\x1b[31m" },   // Threat  → Red
    MarkerColor { id: 60, bold: "\x1b[1;32m", normal: "\x1b[32m" },   // Liking  → Green
    MarkerColor { id: 62, bold: "\x1b[1;34m", normal: "\x1b[34m" },   // Antireward → Blue
    MarkerColor { id: 63, bold: "\x1b[1;33m", normal: "\x1b[33m" },   // Own     → Yellow
    MarkerColor { id: 64, bold: "\x1b[1;35m", normal: "\x1b[35m" },   // Novelty → Magenta
    MarkerColor { id: 65, bold: "\x1b[1;36m", normal: "\x1b[36m" },   // Care    → Cyan
    MarkerColor { id: 66, bold: "\x1b[1;31m", normal: "\x1b[31m" },   // Rage    → Red (bold diff)
    MarkerColor { id: 67, bold: "\x1b[1;35m", normal: "\x1b[35m" },   // Panic   → Magenta
    MarkerColor { id: 68, bold: "\x1b[1;37m", normal: "\x1b[37m" },   // Assumption → White
];

