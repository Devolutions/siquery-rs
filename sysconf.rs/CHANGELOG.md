# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [0.3.1] - 2017-09-04

### Fixed
- Added missing dependencies for Windows

### Added
- Integration with AppVeyor to catch build problems for Windows

## [0.3.0] - 2017-08-29

### Added
- A `page` module for querying information related to memory pages.

### Changed
- Moved the `sysconf` function and related types to their own `raw` module.
- The `sysconf` function now returns `isize` instead of `c_long`. `c_long`
  (which is what `libc`'s `sysconf` returns) is always 64 bits on 64-bit
  platforms, so this is safe - we will never cast the return value of `sysconf`
  to a smaller type, losing precision.
- Renamed `SysconfError::Invalid` to `SysconfError::Unsupported` to reflect the
  fact that the error doesn't indicate that the requested variable can never be
  valid, but only that it is not supported on the current platform.

## [0.2.0] - 2017-07-22

### Added
- This CHANGELOG file
- A couple basic tests

### Changed
- `sysconf` function now returns c_long instead of i64. This potentially breaks backwards
  compatability but is necessary to work with 32-bit OS's
