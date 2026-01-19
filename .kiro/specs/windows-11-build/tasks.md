# Windows 11 Build - Implementation Tasks

## Task List

- [x] 1. Update utility functions for Windows support
  - [x] 1.1 Add `isWindows()` function to `scripts/util.py`
  - [x] 1.2 Update `run_cmd()` to handle Windows path conversions
  - [x] 1.3 Add Windows-specific command execution handling
  - [x] 1.4 Write unit tests for Windows utility functions

- [x] 2. Update Rust build configuration
  - [x] 2.1 Modify `rust_targets()` in `scripts/rust.py` to detect Windows
  - [x] 2.2 Ensure Windows target returns `["x86_64-pc-windows-msvc"]`
  - [x] 2.3 Update `rust_env()` if needed for Windows-specific environment variables
  - [x] 2.4 Test target detection on Windows platform

- [x] 3. Implement Windows build function
  - [x] 3.1 Create `build_windows()` function in `scripts/build.py`
  - [x] 3.2 Implement executable copying to build directory
  - [x] 3.3 Add license and README file inclusion
  - [x] 3.4 Handle .exe extension properly

- [x] 4. Implement Windows packaging
  - [x] 4.1 Create ZIP archive using Python's zipfile module
  - [x] 4.2 Include qchat.exe in ZIP
  - [x] 4.3 Include LICENSE.MIT, LICENSE.APACHE, and README.md
  - [x] 4.4 Name output as `qchat-windows-x64.zip`

- [x] 5. Implement SHA256 checksum generation
  - [x] 5.1 Create `generate_sha_windows()` function
  - [x] 5.2 Use hashlib for cross-platform hash generation
  - [x] 5.3 Write checksum to `.sha256` file
  - [x] 5.4 Format checksum output correctly (hash + filename)

- [x] 6. Update build_chat_bin for Windows
  - [x] 6.1 Add Windows platform detection in `build_chat_bin()`
  - [x] 6.2 Handle .exe extension for Windows binaries
  - [x] 6.3 Copy binary from target directory to build directory
  - [x] 6.4 Return correct path with .exe extension

- [x] 7. Update main build function
  - [x] 7.1 Add Windows branch in `build()` function
  - [x] 7.2 Call `build_windows()` when on Windows platform
  - [x] 7.3 Pass appropriate parameters (no signing initially)
  - [x] 7.4 Ensure proper error handling

- [x] 8. Add Windows constants
  - [x] 8.1 Add Windows-specific constants to `scripts/const.py` if needed
  - [x] 8.2 Define Windows binary names and paths
  - [x] 8.3 Document Windows-specific configuration

- [x] 9. Update documentation
  - [x] 9.1 Add Windows build instructions to README.md
  - [x] 9.2 Document prerequisites (Visual Studio Build Tools, Rust MSVC)
  - [x] 9.3 Add troubleshooting section for Windows builds
  - [x] 9.4 Document how to verify SHA256 checksums on Windows

- [x] 10. Create Windows build script wrapper
  - [x] 10.1 Create `scripts/build-windows.bat` or `.ps1` script
  - [x] 10.2 Check for required dependencies (Rust, Python)
  - [x] 10.3 Call Python build script with appropriate arguments
  - [x] 10.4 Display helpful error messages

- [ ] 11. Test local Windows build
  - [ ] 11.1 Test compilation on Windows 11
  - [ ] 11.2 Verify executable runs correctly
  - [ ] 11.3 Test with Windows Terminal
  - [ ] 11.4 Verify file paths and AppData usage

- [x] 12. Add CI/CD Windows workflow
  - [x] 12.1 Create or update GitHub Actions workflow
  - [x] 12.2 Add Windows runner job
  - [x] 12.3 Install Rust with MSVC target
  - [x] 12.4 Run build and tests
  - [x] 12.5 Upload Windows artifacts

- [ ] 13. Implement optional code signing (future)
  - [ ] 13.1* Create `WindowsSigner` class
  - [ ] 13.2* Implement Authenticode signing with signtool
  - [ ] 13.3* Add certificate path and password handling
  - [ ] 13.4* Verify signature after signing
  - [ ] 13.5* Document signing process

- [ ] 14. Add integration tests
  - [ ] 14.1 Create Windows-specific test suite
  - [ ] 14.2 Test ZIP extraction and contents
  - [ ] 14.3 Test SHA256 verification
  - [ ] 14.4 Test executable launch and basic commands

- [ ] 15. Update Cross.toml for Windows (if needed)
  - [ ] 15.1 Review Cross.toml configuration
  - [ ] 15.2 Add Windows-specific build settings if cross-compiling
  - [ ] 15.3 Test cross-compilation setup

## Task Dependencies

```
1 (Utils) → 2 (Rust Config) → 6 (build_chat_bin) → 3 (build_windows) → 4 (Packaging)
                                                                        ↓
                                                                    5 (SHA256)
                                                                        ↓
                                                                    7 (Main build)
                                                                        ↓
                                                                    11 (Local Test)
                                                                        ↓
                                                                    12 (CI/CD)

9 (Docs) - Can be done in parallel
10 (Script wrapper) - Can be done after 7
13 (Signing) - Optional, can be done later
14 (Integration tests) - After 11
15 (Cross.toml) - Optional, if needed
```

## Testing Checklist

### Unit Tests
- [ ] Test `isWindows()` returns True on Windows
- [ ] Test `rust_targets()` returns correct Windows target
- [ ] Test `generate_sha_windows()` produces valid SHA256
- [ ] Test ZIP creation includes all required files
- [ ] Test path handling with Windows separators

### Integration Tests
- [ ] Build completes without errors on Windows 11
- [ ] Generated ZIP can be extracted
- [ ] Executable runs and shows version
- [ ] SHA256 checksum matches file
- [ ] All tests pass on Windows

### Manual Testing
- [ ] Install on clean Windows 11 system
- [ ] Run in Windows Terminal
- [ ] Test with PowerShell and CMD
- [ ] Verify AppData directory usage
- [ ] Test with spaces in paths

## Acceptance Criteria

### For Task Completion
- All non-optional tasks marked complete
- All tests passing on Windows
- Documentation updated
- CI/CD producing Windows artifacts
- Manual testing successful

### For Release
- Windows ZIP available for download
- SHA256 checksum provided
- Installation instructions clear
- No critical bugs reported
- Performance comparable to other platforms

## Notes

- Tasks marked with `*` are optional and can be deferred
- Focus on core functionality first (tasks 1-12)
- Code signing (task 13) can be added in a future release
- Ensure backward compatibility with existing macOS/Linux builds
- Test on both Windows 10 and Windows 11 if possible

## Estimated Effort

- **Core Implementation** (Tasks 1-8): 2-3 days
- **Documentation** (Task 9-10): 0.5 day
- **Testing** (Tasks 11, 14): 1-2 days
- **CI/CD Integration** (Task 12): 1 day
- **Optional Signing** (Task 13): 1-2 days (if implemented)

**Total Estimated Time**: 4-6 days (excluding optional tasks)
