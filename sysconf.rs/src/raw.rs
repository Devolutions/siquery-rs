//! Directly the `sysconf` function.
//!
//! Unlike the other modules in this crate, which provide convenience wrappers to query various
//! configuration information, the `raw` module provides the ability to call the `sysconf` function
//! directly.

extern crate libc;
use self::libc::c_int;
use core::result;

pub type Result<T> = result::Result<T, SysconfError>;

/// The error returned by `sysconf`.
#[derive(Debug)]
pub enum SysconfError {
    /// The queried variable is unsupported on the current system.
    Unsupported,
}

macro_rules! sc { ($var:ident) => (libc::$var as isize) }

/// The variables that can be queried using `sysconf`.
///
/// Each variable corresponds to a sysconf variable defined by POSIX. For example, `ScArgMax`
/// corresponds to the `_SC_ARG_MAX` variable, and so on.
///
/// Different variables may have been added at different times, and as a result, some variables
/// may not be supported on older systems. In these cases, `sysconf` will return an error.
pub enum SysconfVariable {
    ScArgMax = sc!(_SC_ARG_MAX),
    ScChildMax = sc!(_SC_CHILD_MAX),
    ScClkTck = sc!(_SC_CLK_TCK),
    ScNgroupsMax = sc!(_SC_NGROUPS_MAX),
    ScOpenMax = sc!(_SC_OPEN_MAX),
    ScStreamMax = sc!(_SC_STREAM_MAX),
    ScTznameMax = sc!(_SC_TZNAME_MAX),
    ScJobControl = sc!(_SC_JOB_CONTROL),
    ScSavedIds = sc!(_SC_SAVED_IDS),
    ScRealtimeSignals = sc!(_SC_REALTIME_SIGNALS),
    ScPriorityScheduling = sc!(_SC_PRIORITY_SCHEDULING),
    ScTimers = sc!(_SC_TIMERS),
    ScAsynchronousIo = sc!(_SC_ASYNCHRONOUS_IO),
    ScPrioritizedIo = sc!(_SC_PRIORITIZED_IO),
    ScSynchronizedIo = sc!(_SC_SYNCHRONIZED_IO),
    ScFsync = sc!(_SC_FSYNC),
    ScMappedFiles = sc!(_SC_MAPPED_FILES),
    ScMemlock = sc!(_SC_MEMLOCK),
    ScMemlockRange = sc!(_SC_MEMLOCK_RANGE),
    ScMemoryProtection = sc!(_SC_MEMORY_PROTECTION),
    ScMessagePassing = sc!(_SC_MESSAGE_PASSING),
    ScSemaphores = sc!(_SC_SEMAPHORES),
    ScSharedMemoryObjects = sc!(_SC_SHARED_MEMORY_OBJECTS),
    ScAioListioMax = sc!(_SC_AIO_LISTIO_MAX),
    ScAioMax = sc!(_SC_AIO_MAX),
    ScAioPrioDeltaMax = sc!(_SC_AIO_PRIO_DELTA_MAX),
    ScDelaytimerMax = sc!(_SC_DELAYTIMER_MAX),
    ScMqOpenMax = sc!(_SC_MQ_OPEN_MAX),
    ScVersion = sc!(_SC_VERSION),
    ScPagesize = sc!(_SC_PAGESIZE),
    ScRtsigMax = sc!(_SC_RTSIG_MAX),
    ScSemNsemsMax = sc!(_SC_SEM_NSEMS_MAX),
    ScSemValueMax = sc!(_SC_SEM_VALUE_MAX),
    ScSigqueueMax = sc!(_SC_SIGQUEUE_MAX),
    ScTimerMax = sc!(_SC_TIMER_MAX),
    ScBcBaseMax = sc!(_SC_BC_BASE_MAX),
    ScBcDimMax = sc!(_SC_BC_DIM_MAX),
    ScBcScaleMax = sc!(_SC_BC_SCALE_MAX),
    ScBcStringMax = sc!(_SC_BC_STRING_MAX),
    ScCollWeightsMax = sc!(_SC_COLL_WEIGHTS_MAX),
    ScExprNestMax = sc!(_SC_EXPR_NEST_MAX),
    ScLineMax = sc!(_SC_LINE_MAX),
    ScReDupMax = sc!(_SC_RE_DUP_MAX),
    Sc2Version = sc!(_SC_2_VERSION),
    Sc2CBind = sc!(_SC_2_C_BIND),
    Sc2CDev = sc!(_SC_2_C_DEV),
    Sc2FortDev = sc!(_SC_2_FORT_DEV),
    Sc2FortRun = sc!(_SC_2_FORT_RUN),
    Sc2SwDev = sc!(_SC_2_SW_DEV),
    Sc2Localedef = sc!(_SC_2_LOCALEDEF),
    ScNprocessorsConf = sc!(_SC_NPROCESSORS_CONF),
    ScNprocessorsOnln = sc!(_SC_NPROCESSORS_ONLN),
    Sc2CharTerm = sc!(_SC_2_CHAR_TERM),
    Sc2CVersion = 96, // TODO(joshlf): Switch to a libc constant once it's added
    Sc2Upe = sc!(_SC_2_UPE),
    ScXbs5Ilp32Off32 = sc!(_SC_XBS5_ILP32_OFF32),
    ScXbs5Ilp32Offbig = sc!(_SC_XBS5_ILP32_OFFBIG),
    ScXbs5LpbigOffbig = sc!(_SC_XBS5_LPBIG_OFFBIG),
}

/// Query the system's configuration.
///
/// `sysconf` calls the POSIX `sysconf` function from the system's `libc`. The result is either an
/// `isize` representing the queried variable's value, or an error indicating that the queried
/// variable is not supported on the current system.
pub fn sysconf(name: SysconfVariable) -> Result<isize> {
    // TODO(joshlf): What error conditions are possible?
    // - On Linux, EINVAL implies that an unknown variable was requested, and no other errors are
    //   possible.
    // - On Mac, -1 with errno changed means that there was a failure to query the configuration,
    //   while -1 with errno unchanged means that an unknown variable was requested. It's not clear
    //   to me (joshlf) that it's always possible to distinguish between these two scenarios.
    match unsafe { libc::sysconf(name as c_int) } {
        -1 => Err(SysconfError::Unsupported),
        ret => Ok(ret as isize),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sysconf() {
        let result = sysconf(SysconfVariable::ScPagesize);
        assert!(result.is_ok())
    }
}
