mod point;
mod range;
mod rect;
mod size;
pub mod string;

pub use point::*;
pub use range::*;
pub use rect::*;
pub use size::*;

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
#[repr(u32)]
pub enum LogSeverity {
    Default = cef_sys::cef_log_severity_t_LOGSEVERITY_DEFAULT,
    // Verbose = cef_sys::cef_log_severity_t_LOGSEVERITY_VERBOSE,
    Debug = cef_sys::cef_log_severity_t_LOGSEVERITY_DEBUG,
    Info = cef_sys::cef_log_severity_t_LOGSEVERITY_INFO,
    Warning = cef_sys::cef_log_severity_t_LOGSEVERITY_WARNING,
    Error = cef_sys::cef_log_severity_t_LOGSEVERITY_ERROR,
    Fatal = cef_sys::cef_log_severity_t_LOGSEVERITY_FATAL,
    Disable = cef_sys::cef_log_severity_t_LOGSEVERITY_DISABLE,
}
