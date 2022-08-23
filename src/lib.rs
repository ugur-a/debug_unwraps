#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::fmt;

/// Extension trait providing debug only checking of item validity
pub trait DebugUnwrapExt {
    /// Expected type after performing an unwrap
    type Value;

    /// Returns the contained `Some()` or `Ok()` variant without checking
    /// the discriminant only in Release mode.
    ///
    /// # Panics
    /// When debug assertions are enabled this function will panic if the
    /// value is not `Some()` or `Ok()`.
    ///
    /// # Safety
    /// Calling this method on `None` or `Err()` is undefined behavior when
    /// debug assertions are disabled.
    unsafe fn debug_unwrap_unchecked(self) -> Self::Value;

    /// Returns the contained `Some()` or `Ok()` variant without checking
    /// the discriminant only in Release mode.
    ///
    /// # Panics
    /// When debug assertions are enabled this function will panic with the
    /// provided `msg`.
    ///
    /// # Safety
    /// Calling this method on `None` or `Err()` is undefined behavior when
    /// debug assertions are disabled.
    unsafe fn debug_expect_unchecked(self, msg: &str) -> Self::Value;
}

/// Extension trait providing debug only checking of error validity
pub trait DebugUnwrapErrExt {
    /// Expected error type after unwrap
    type ErrorType;

    /// Returns the contained `Err()` variant without checking
    /// the discriminant only in Release mode.
    ///
    /// # Panics
    /// When debug assertions are enabled this function will panic if the
    /// Result is not `Result::Err()`.
    ///
    /// # Safety
    /// Calling this method on `None` or `Err()` is undefined behavior when
    /// debug assertions are disabled.
    unsafe fn debug_unwrap_err_unchecked(self) -> Self::ErrorType;

    /// Returns the contained `Err()` variant without checking
    /// the discriminant only in Release mode.
    ///
    /// # Panics
    /// When debug assertions are enabled this function will panic if the
    /// Result is not `Result::Err()` and will print the provided `msg`.
    ///
    /// # Safety
    /// Calling this method on `None` or `Err()` is undefined behavior when
    /// debug assertions are disabled.
    unsafe fn debug_expect_err_unchecked(self, msg: &str) -> Self::ErrorType;
}

impl<T> DebugUnwrapExt for Option<T> {
    type Value = T;

    #[inline]
    #[track_caller]
    unsafe fn debug_unwrap_unchecked(self) -> Self::Value {
        #[cfg(debug_assertions)]
        {
            self.unwrap()
        }
        #[cfg(not(debug_assertions))]
        {
            self.unwrap_unchecked()
        }
    }

    #[inline]
    #[track_caller]
    #[cfg_attr(not(debug_assertions), allow(unused_variables))]
    unsafe fn debug_expect_unchecked(self, msg: &str) -> Self::Value {
        #[cfg(debug_assertions)]
        {
            self.expect(msg)
        }
        #[cfg(not(debug_assertions))]
        {
            self.unwrap_unchecked()
        }
    }
}

impl<T, E> DebugUnwrapExt for Result<T, E>
where
    E: fmt::Debug,
{
    type Value = T;

    #[inline]
    #[track_caller]
    unsafe fn debug_unwrap_unchecked(self) -> Self::Value {
        #[cfg(debug_assertions)]
        {
            self.unwrap()
        }
        #[cfg(not(debug_assertions))]
        {
            self.unwrap_unchecked()
        }
    }

    #[inline]
    #[track_caller]
    #[cfg_attr(not(debug_assertions), allow(unused_variables))]
    unsafe fn debug_expect_unchecked(self, msg: &str) -> Self::Value {
        #[cfg(debug_assertions)]
        {
            self.expect(msg)
        }
        #[cfg(not(debug_assertions))]
        {
            self.unwrap_unchecked()
        }
    }
}

impl<T, E> DebugUnwrapErrExt for Result<T, E>
where
    T: fmt::Debug,
{
    type ErrorType = E;

    #[inline]
    #[track_caller]
    unsafe fn debug_unwrap_err_unchecked(self) -> Self::ErrorType {
        #[cfg(debug_assertions)]
        {
            self.unwrap_err()
        }
        #[cfg(not(debug_assertions))]
        {
            self.unwrap_err_unchecked()
        }
    }

    #[inline]
    #[track_caller]
    #[cfg_attr(not(debug_assertions), allow(unused_variables))]
    unsafe fn debug_expect_err_unchecked(self, msg: &str) -> Self::ErrorType {
        #[cfg(debug_assertions)]
        {
            self.expect_err(msg)
        }
        #[cfg(not(debug_assertions))]
        {
            self.unwrap_err_unchecked()
        }
    }
}
