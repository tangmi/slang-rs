#![allow(overflowing_literals)]

/// SLANG_OK indicates success, and is equivalent to SLANG_MAKE_SUCCESS(SLANG_FACILITY_WIN_GENERAL, 0)
const S_OK: slang_sys::SlangResult = 0x00000000;
/// SLANG_FAIL is the generic failure code - meaning a serious error occurred and the call couldn't complete
const E_FAIL: slang_sys::SlangResult = 0x80004005;

/// Functionality is not implemented
const E_NOT_IMPLEMENTED: slang_sys::SlangResult = 0x80004001;
/// Interface not be found
const E_NO_INTERFACE: slang_sys::SlangResult = 0x80004002;
/// Operation was aborted (did not correctly complete)
const E_ABORT: slang_sys::SlangResult = 0x80004004;

/// Indicates that a handle passed in as parameter to a method is invalid.
const E_INVALID_HANDLE: slang_sys::SlangResult = 0x80070006;
/// Indicates that an argument passed in as parameter to a method is invalid.
const E_INVALID_ARG: slang_sys::SlangResult = 0x80070057;
/// Operation could not complete - ran out of memory
const E_OUT_OF_MEMORY: slang_sys::SlangResult = 0x8007000e;

// Supplied buffer is too small to be able to complete
const SLANG_E_BUFFER_TOO_SMALL: slang_sys::SlangResult = 0x82000001;
/// Used to identify a Result that has yet to be initialized.
/// It defaults to failure such that if used incorrectly will fail, as similar in concept to using an uninitialized variable.
const SLANG_E_UNINITIALIZED: slang_sys::SlangResult = 0x82000002;
/// Returned from an async method meaning the output is invalid (thus an error), but a result for the request is pending, and will be returned on a subsequent call with the async handle.
const SLANG_E_PENDING: slang_sys::SlangResult = 0x82000003;
/// Indicates a file/resource could not be opened
const SLANG_E_CANNOT_OPEN: slang_sys::SlangResult = 0x82000004;
/// Indicates a file/resource could not be found
const SLANG_E_NOT_FOUND: slang_sys::SlangResult = 0x82000005;
/// An unhandled internal failure (typically from unhandled exception)
const SLANG_E_INTERNAL_FAIL: slang_sys::SlangResult = 0x82000006;

// TODO impl Display for SlangResult wrapper? can parse severity, facility, code
pub type Result = ::std::result::Result<(), Error>;

// TODO: unsure how to get a From<SlangResult> impl since I own neither SlangResult (i32) or Result
pub fn into_result(slang_result: slang_sys::SlangResult) -> Result {
    if slang_result == 0 {
        Ok(())
    } else {
        Err(match slang_result {
            x if x == E_FAIL => Error::Fail,
            x if x == E_NOT_IMPLEMENTED => Error::NotImplemented,
            x if x == E_NO_INTERFACE => Error::NoInterface,
            x if x == E_ABORT => Error::Abort,
            x if x == E_INVALID_HANDLE => Error::InvalidHandle,
            x if x == E_INVALID_ARG => Error::InvalidArg,
            x if x == E_OUT_OF_MEMORY => Error::OutOfMemory,
            x if x == SLANG_E_BUFFER_TOO_SMALL => Error::SlangErrorBufferTooSmall,
            x if x == SLANG_E_UNINITIALIZED => Error::SlangErrorUninitialized,
            x if x == SLANG_E_PENDING => Error::SlangErrorPending,
            x if x == SLANG_E_CANNOT_OPEN => Error::SlangErrorCannotOpen,
            x if x == SLANG_E_NOT_FOUND => Error::SlangErrorNotFound,
            x if x == SLANG_E_INTERNAL_FAIL => Error::SlangErrorInternalFail,
            result => Error::Unknown(result),
        })
    }
}

#[derive(Debug)]
pub enum Error {
    Fail,
    NotImplemented,
    NoInterface,
    Abort,
    InvalidHandle,
    InvalidArg,
    OutOfMemory,
    SlangErrorBufferTooSmall,
    SlangErrorUninitialized,
    SlangErrorPending,
    SlangErrorCannotOpen,
    SlangErrorNotFound,
    SlangErrorInternalFail,
    Unknown(slang_sys::SlangResult),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Fail => write!(f, "E_FAIL"),
            Error::NotImplemented => write!(f, "E_NOT_IMPLEMENTED"),
            Error::NoInterface => write!(f, "E_NO_INTERFACE"),
            Error::Abort => write!(f, "E_ABORT"),
            Error::InvalidHandle => write!(f, "E_INVALID_HANDLE"),
            Error::InvalidArg => write!(f, "E_INVALID_ARG"),
            Error::OutOfMemory => write!(f, "E_OUT_OF_MEMORY"),
            Error::SlangErrorBufferTooSmall => write!(f, "SLANG_E_BUFFER_TOO_SMALL"),
            Error::SlangErrorUninitialized => write!(f, "SLANG_E_UNINITIALIZED"),
            Error::SlangErrorPending => write!(f, "SLANG_E_PENDING"),
            Error::SlangErrorCannotOpen => write!(f, "SLANG_E_CANNOT_OPEN"),
            Error::SlangErrorNotFound => write!(f, "SLANG_E_NOT_FOUND"),
            Error::SlangErrorInternalFail => write!(f, "SLANG_E_INTERNAL_FAIL"),
            Error::Unknown(result) => write!(f, "Unknown HResult: {}", result),
        }
    }
}
