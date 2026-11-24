# INSTALLATION

SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>

SPDX-License-Identifier: GPL 3.0

---

GPUPCB is available as prebuilt desktop packages for Linux, macOS, and Windows.

Here you can download releases:
https://github.com/wecamo-io/gpupcb/releases

## Supported Platforms

| Operating System | Architecture | Package Formats |
|---|---|---|
| Linux | x86_64 (AMD64) | `.deb`, `.rpm`, `.AppImage` |
| Linux | ARM64 (AArch64) | `.deb`, `.rpm`, `.AppImage` |
| macOS | Apple Silicon (ARM64) | `.dmg`, `.app.tar.gz` |
| macOS | Intel (x64) | `.dmg`, `.app.tar.gz` |
| Windows | x64 | `.exe`, `.msi` |


## Linux

GPUPCB provides DEB, RPM and AppImage packages for both AMD64 and ARM64 architectures.

### Debian / Ubuntu

| Architecture | Package |
|---|---|
| AMD64 (x86_64) | `GPUPCB_0.1.0_amd64.deb` |
| ARM64 (AArch64) | `GPUPCB_0.1.0_arm64.deb` |

**AMD64 installation:**
```bash
sudo apt install ./GPUPCB_0.1.0_amd64.deb
```

**ARM64 installation:**
```bash
sudo apt install ./GPUPCB_0.1.0_arm64.deb
```

### Red Hat-based distributions

| Architecture | Package |
|---|---|
| AMD64 (x86_64) | `GPUPCB-0.1.0-1.x86_64.rpm` |
| ARM64 (AArch64) | `GPUPCB-0.1.0-1.aarch64.rpm` |

**AMD64 installation:**
```bash
sudo dnf install ./GPUPCB-0.1.0-1.x86_64.rpm
```

**ARM64 installation:**
```bash
sudo dnf install ./GPUPCB-0.1.0-1.aarch64.rpm
```

### AppImage
AppImage packages are available for both architectures.

| Architecture | Package |
|---|---|
| AMD64 (x86_64) | `GPUPCB_0.1.0_amd64.AppImage` |
| ARM64 (AArch64) | `GPUPCB_0.1.0_aarch64.AppImage` |

First make the file executable:
```bash
chmod +x GPUPCB_*.AppImage
```

Then you can run the app:
```bash
./GPUPCB_0.1.0_amd64.AppImage
```
Or in the ARM64 systems:
```bash
./GPUPCB_0.1.0_aarch64.AppImage
```

## macOS
There are packages for both Apple Silicon and Intel Macs.

### Apple Silicon (ARM64)

| Package format | Filename |
|---|---|
| dmg | `GPUPCB_0.1.0_aarch64.dmg` |
| tar.gz | `GPUPCB_0.1.0_aarch64.app.tar.gz` |

### Intel (x86_64)

| Package format | Filename |
|---|---|
| dmg | `GPUPCB_0.1.0_x64.dmg` |
| tar.gz | `GPUPCB_0.1.0_x64.app.tar.gz` |

In both architectures installation process is similar:

**Installation using dmg**
1. Open the donwnloaded dmg file
2. Drag the GPUPCB application into *Applications* directory
3. Launch the app

**Installation using .tar.gz**
1. Extract the file
2. Move the extracted part to *Applications* directory

## Windows

| Installer | Filename |
|---|---|
| exe | `GPUPCB_0.1.0_x64-setup.exe` |
| msi | `GPUPCB_0.1.0_x64_en-US.msi` |

**Exe installer**
1. Run the donwloaded installer
2. Follow the installation process
3. Launch the app

**Msi installer**
1. Run the donwloaded installer
2. Follow the installation process
3. Launch the app

## UPDATING GPUPCB

1. Visit:
   https://github.com/wecamo-io/gpupcb/releases

2. Download the latest package for your operating system and architecture.

3. Install the new version.

Check the release notes for changes, improvements, and compatibility issues.

## REPORTING INSTALLATION PROBLEMS

If you encounter an installation issue, contact us as stated in SECURITY.md file:

https://github.com/wecamo-io/gpupcb?tab=security-ov-file

Please include:
- Operating system and version.
- CPU architecture.
- Package format.
- Error messages.
