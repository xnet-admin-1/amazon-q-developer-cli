# Windows 11 Build Design

## Overview
This design document outlines the technical approach for enabling Windows 11 builds of the Amazon Q CLI application. The solution extends the existing build infrastructure to support Windows while maintaining consistency with macOS and Linux build processes.

## Architecture

### High-Level Design
```
┌─────────────────────────────────────────────────────────┐
│                   Build Entry Point                      │
│              (scripts/build.py main())                   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ├─── Platform Detection (util.py)
                     │
        ┌────────────┼────────────┬────────────────┐
        │            │            │                │
   ┌────▼───┐  ┌────▼───┐  ┌─────▼─────┐   ┌─────▼──────┐
   │ macOS  │  │ Linux  │  │  Windows  │   │   Cross    │
   │ Build  │  │ Build  │  │   Build   │   │  Platform  │
   └────┬───┘  └────┬───┘  └─────┬─────┘   └─────┬──────┘
        │           │            │                │
        │           │            │                │
   ┌────▼───────────▼────────────▼────────────────▼──────┐
   │         Cargo Build (Rust Compilation)              │
   │    Target: x86_64-pc-windows-msvc                   │
   └────────────────────┬────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
   ┌────▼────┐    ┌─────▼─────┐   ┌────▼─────┐
   │ Package │    │   Sign    │   │ Generate │
   │  (ZIP)  │    │(Optional) │   │  SHA256  │
   └─────────┘    └───────────┘   └──────────┘
```

### Component Design

#### 1. Platform Detection Enhancement
**File**: `scripts/util.py`

Add Windows detection function:
```python
def isWindows() -> bool:
    """Returns True if running on Windows"""
    return platform.system() == "Windows"
```

#### 2. Windows Build Function
**File**: `scripts/build.py`

New function `build_windows()`:
- Copies compiled binary to build directory
- Creates ZIP archive with executable
- Generates SHA256 checksum
- Optionally signs executable with Authenticode

#### 3. Rust Build Configuration
**File**: `scripts/rust.py`

Update `rust_targets()` to detect Windows:
```python
def rust_targets() -> list[str]:
    if isWindows():
        return ["x86_64-pc-windows-msvc"]
    elif isDarwin():
        return ["x86_64-apple-darwin", "aarch64-apple-darwin"]
    else:
        return ["x86_64-unknown-linux-gnu"]
```

#### 4. Windows Packaging
Create distributable package structure:
```
qchat-windows-x64.zip
├── qchat.exe
├── README.md
├── LICENSE.MIT
└── LICENSE.APACHE
```

## Detailed Design

### Build Script Modifications

#### scripts/build.py

**New Function: `build_windows()`**
```python
def build_windows(chat_path: pathlib.Path, signer: Optional[WindowsSigner] = None):
    """
    Creates qchat.zip under the build directory for Windows.
    
    Args:
        chat_path: Path to the compiled executable
        signer: Optional code signer for Authenticode
    """
    # Copy executable to build directory
    chat_dst = BUILD_DIR / f"{CHAT_BINARY_NAME}.exe"
    chat_dst.unlink(missing_ok=True)
    shutil.copy2(chat_path, chat_dst)
    
    # Sign if signer provided
    if signer:
        chat_dst = signer.sign_executable(chat_dst)
    
    # Create ZIP archive
    zip_path = BUILD_DIR / f"{CHAT_BINARY_NAME}-windows-x64.zip"
    zip_path.unlink(missing_ok=True)
    
    info(f"Creating zip output to {zip_path}")
    
    # Use Python's zipfile module (cross-platform)
    import zipfile
    with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
        zipf.write(chat_dst, f"{CHAT_BINARY_NAME}.exe")
        # Add license files
        if pathlib.Path("LICENSE.MIT").exists():
            zipf.write("LICENSE.MIT", "LICENSE.MIT")
        if pathlib.Path("LICENSE.APACHE").exists():
            zipf.write("LICENSE.APACHE", "LICENSE.APACHE")
        if pathlib.Path("README.md").exists():
            zipf.write("README.md", "README.md")
    
    generate_sha_windows(zip_path)
```

**New Function: `generate_sha_windows()`**
```python
def generate_sha_windows(path: pathlib.Path) -> pathlib.Path:
    """Generate SHA256 checksum for Windows builds"""
    import hashlib
    
    sha256_hash = hashlib.sha256()
    with open(path, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    
    sha = sha256_hash.hexdigest()
    sha_path = path.with_name(f"{path.name}.sha256")
    sha_path.write_text(f"{sha}  {path.name}\n")
    info(f"Wrote sha256sum to {sha_path}: {sha}")
    return sha_path
```

**Update `build_chat_bin()` for Windows**
```python
def build_chat_bin(
    release: bool,
    output_name: str | None = None,
    targets: Sequence[str] = [],
):
    package = CHAT_PACKAGE_NAME
    
    # ... existing code ...
    
    # Windows-specific handling
    if isWindows():
        target = targets[0]
        exe_name = f"{package}.exe"
        target_path = pathlib.Path("target") / target / target_subdir / exe_name
        out_path = BUILD_DIR / "bin" / f"{(output_name or package)}-{target}.exe"
        out_path.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(target_path, out_path)
        return out_path
    # ... rest of existing code ...
```

**Update main `build()` function**
```python
def build(
    release: bool,
    stage_name: str | None = None,
    run_lints: bool = True,
    run_test: bool = True,
):
    # ... existing setup code ...
    
    # Platform-specific build
    if isDarwin():
        build_macos(chat_path, signing_data)
    elif isWindows():
        # Windows signing not implemented yet
        build_windows(chat_path, signer=None)
    else:
        build_linux(chat_path, gpg_signer)
```

#### scripts/util.py

**Add Windows detection**
```python
def isWindows() -> bool:
    """Check if the current platform is Windows"""
    return platform.system() == "Windows"
```

**Update command execution for Windows**
```python
def run_cmd(args: Args, env: Env | None = None, cwd: Cwd | None = None):
    """Execute command with Windows compatibility"""
    # Convert Path objects to strings for Windows compatibility
    args = [str(arg) for arg in args]
    if cwd:
        cwd = str(cwd)
    
    # ... rest of existing implementation ...
```

#### scripts/rust.py

**Update target detection**
```python
def rust_targets() -> list[str]:
    """Get Rust compilation targets for current platform"""
    if isWindows():
        return ["x86_64-pc-windows-msvc"]
    elif isDarwin():
        return ["x86_64-apple-darwin", "aarch64-apple-darwin"]
    else:
        return ["x86_64-unknown-linux-gnu"]
```

### Code Signing (Optional)

#### Windows Authenticode Signing

**New Class: `WindowsSigner`**
```python
class WindowsSigner:
    """Handle Windows Authenticode signing"""
    
    def __init__(self, cert_path: str, cert_password: str):
        self.cert_path = cert_path
        self.cert_password = cert_password
    
    def sign_executable(self, exe_path: pathlib.Path) -> pathlib.Path:
        """
        Sign Windows executable with Authenticode
        
        Requires signtool.exe from Windows SDK
        """
        info(f"Signing {exe_path.name}")
        
        # Use signtool from Windows SDK
        signtool = self._find_signtool()
        
        run_cmd([
            signtool,
            "sign",
            "/f", self.cert_path,
            "/p", self.cert_password,
            "/tr", "http://timestamp.digicert.com",
            "/td", "sha256",
            "/fd", "sha256",
            str(exe_path)
        ])
        
        # Verify signature
        run_cmd([signtool, "verify", "/pa", str(exe_path)])
        
        return exe_path
    
    def _find_signtool(self) -> str:
        """Locate signtool.exe in Windows SDK"""
        # Common locations for signtool
        sdk_paths = [
            r"C:\Program Files (x86)\Windows Kits\10\bin\*\x64\signtool.exe",
            r"C:\Program Files\Windows Kits\10\bin\*\x64\signtool.exe",
        ]
        
        import glob
        for pattern in sdk_paths:
            matches = glob.glob(pattern)
            if matches:
                return matches[0]
        
        raise FileNotFoundError("signtool.exe not found. Install Windows SDK.")
```

### Testing Strategy

#### Unit Tests
- Test Windows path handling in `util.py`
- Test ZIP creation in `build_windows()`
- Test SHA256 generation
- Mock Windows-specific APIs

#### Integration Tests
- Build on Windows 11 VM
- Verify executable runs
- Test with Windows Terminal
- Verify file paths and AppData usage

#### Platform-Specific Tests
```python
@pytest.mark.skipif(not isWindows(), reason="Windows-only test")
def test_windows_build():
    """Test Windows build process"""
    result = build_chat_bin(
        release=True,
        output_name="qchat",
        targets=["x86_64-pc-windows-msvc"]
    )
    assert result.exists()
    assert result.suffix == ".exe"
```

## Data Flow

### Build Process Flow
1. **Initialization**: Detect platform (Windows)
2. **Compilation**: Run `cargo build --target x86_64-pc-windows-msvc`
3. **Binary Location**: `target/x86_64-pc-windows-msvc/release/chat_cli.exe`
4. **Copy**: Move to `build/bin/qchat-x86_64-pc-windows-msvc.exe`
5. **Package**: Create ZIP with executable and licenses
6. **Checksum**: Generate SHA256 hash
7. **Output**: `build/qchat-windows-x64.zip` and `.sha256` file

### File Paths
- **Build Output**: `build/bin/qchat-x86_64-pc-windows-msvc.exe`
- **Package**: `build/qchat-windows-x64.zip`
- **Checksum**: `build/qchat-windows-x64.zip.sha256`

## Error Handling

### Build Failures
- **Missing MSVC**: Clear error message directing to Visual Studio installation
- **Compilation errors**: Display Rust compiler output
- **Packaging errors**: Validate ZIP creation and file inclusion

### Runtime Errors
- **Missing dependencies**: Check for required DLLs
- **Permission errors**: Handle Windows UAC and file permissions
- **Path errors**: Validate Windows path formats

## Security Considerations

### Code Signing
- Store certificates securely (not in repository)
- Use environment variables or secret management
- Implement signing only in CI/CD, not local builds

### Binary Distribution
- Provide SHA256 checksums for verification
- Document signature verification process
- Use HTTPS for distribution

### Build Environment
- Use clean build environments
- Verify dependencies before compilation
- Scan for vulnerabilities in dependencies

## Performance Considerations

### Build Time
- Expected: 5-10 minutes for release build
- Parallel compilation enabled by default
- Incremental builds for development

### Binary Size
- Expected: 15-25 MB (release build)
- Strip debug symbols in release mode
- Consider UPX compression (optional)

## Deployment Strategy

### Local Development
```bash
# Install Rust with MSVC target
rustup target add x86_64-pc-windows-msvc

# Build
python scripts/main.py build --release

# Output in build/ directory
```

### CI/CD Integration
- Add Windows runner to GitHub Actions
- Build on `windows-latest` image
- Upload artifacts to releases
- Run tests on Windows

## Alternatives Considered

### 1. Cross-Compilation from Linux
**Rejected**: Complex setup, harder to debug Windows-specific issues

### 2. MSI Installer
**Deferred**: ZIP distribution is simpler for initial release, MSI can be added later

### 3. Windows Store Distribution
**Rejected**: Out of scope, requires additional packaging and certification

## Dependencies

### Build Dependencies
- Rust 1.87.0 with MSVC toolchain
- Visual Studio Build Tools 2019+
- Python 3.8+
- Windows SDK (for signing)

### Runtime Dependencies
- Windows 11 (21H2+)
- Visual C++ Redistributable (bundled with Windows)

## Correctness Properties

### Property 1: Build Reproducibility
**Description**: Building the same source code twice produces identical binaries (excluding timestamps)

**Test Strategy**: 
- Build twice with same inputs
- Compare binary hashes (excluding PE timestamp)
- Verify deterministic compilation

### Property 2: Platform Detection Accuracy
**Description**: Build system correctly identifies Windows platform and selects appropriate targets

**Test Strategy**:
- Mock platform.system() to return "Windows"
- Verify rust_targets() returns ["x86_64-pc-windows-msvc"]
- Verify build_windows() is called instead of build_macos/build_linux

### Property 3: Package Integrity
**Description**: Generated ZIP contains all required files and is not corrupted

**Test Strategy**:
- Extract ZIP and verify contents
- Check for qchat.exe, LICENSE files, README
- Verify ZIP can be opened by Windows Explorer
- Validate SHA256 checksum matches

### Property 4: Executable Validity
**Description**: Compiled executable is a valid Windows PE file and runs without errors

**Test Strategy**:
- Use `dumpbin` to verify PE format
- Check for required DLL dependencies
- Execute with `--version` flag
- Verify exit code is 0

### Property 5: Path Handling Correctness
**Description**: All file paths use Windows-compatible separators and formats

**Test Strategy**:
- Test with various Windows paths (C:\, UNC paths, relative paths)
- Verify no Unix-style paths (/) in Windows builds
- Check AppData path resolution
- Test with spaces and special characters in paths

## Open Questions

1. **Code Signing**: Do we have access to an Authenticode certificate?
   - **Resolution**: Implement signing as optional, document self-signing for testing

2. **Installer Format**: Should we provide MSI or NSIS installer?
   - **Resolution**: Start with ZIP, add installer in future iteration

3. **Auto-Updates**: How should Windows users receive updates?
   - **Resolution**: Out of scope for initial release, manual download

4. **Windows Terminal Integration**: Should we add context menu integration?
   - **Resolution**: Document manual integration, consider automation later

## Future Enhancements

1. **MSI Installer**: Create Windows Installer package
2. **Auto-Updates**: Implement update mechanism
3. **Windows ARM64**: Support ARM-based Windows devices
4. **Chocolatey Package**: Distribute via Chocolatey package manager
5. **Winget Package**: Add to Windows Package Manager
6. **Shell Integration**: Add to Windows Terminal profiles automatically

## References

- [Rust Windows MSVC Target](https://doc.rust-lang.org/rustc/platform-support/pc-windows-msvc.html)
- [Windows Code Signing](https://docs.microsoft.com/en-us/windows/win32/seccrypto/cryptography-tools)
- [Python zipfile module](https://docs.python.org/3/library/zipfile.html)
- [GitHub Actions Windows Runners](https://docs.github.com/en/actions/using-github-hosted-runners/about-github-hosted-runners#supported-runners-and-hardware-resources)
