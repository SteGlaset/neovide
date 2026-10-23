# Experimental Windows startup improvements

This branch is rebased onto upstream main at `882a51b0c3d0e83481a0b188e6853a00261e5757` and contains three independent changes:

1. A Git-pinned winit 0.30.13 backport of [PR #4575](https://github.com/rust-windowing/winit/pull/4575).
2. A Windows-only initial maximized-window presentation path using DWM cloaking.
3. An optional native launcher that preserves hidden startup without a PowerShell process.


## Patched dependency

The Cargo override uses [SteGlaset/winit](https://github.com/SteGlaset/winit/tree/fix/windows-hidden-startup-0.30),
pinned to commit "1b0ee1d7c0518efe42eea15474341fb5b27efa92" in both the manifest and lockfile.
It preserves the winit 0.30.13 API and existing registry dependency versions.
No winit sources are vendored in this repository. The first build requires Git
access to that revision; subsequent builds can use Cargo's local cache.

## Build

Install the normal Neovide Windows build prerequisites, including Rust and MSVC.
Build from an x64 Visual Studio Developer PowerShell:

```powershell
cargo build --release --locked
.\extra\windows\build-fast-launcher.ps1
```

The optional `neovide-fast-launch.exe` is produced alongside `neovide.exe` in
`target/release`. Keep them together. It locates Neovide relative to its own path,
uses OpenGL, forwards additional command-line arguments, and inherits the working
directory and environment. It does not change installation, PATH, associations,
or Neovim configuration.

## Initial window presentation

For the initial window requested in maximized state, Neovide sets `DWMWA_CLOAK`
and temporarily sets `DWMWA_TRANSITIONS_FORCEDISABLED`. While cloaked, it makes
the native window visible so Windows can apply maximization and calculate its
final client size before construction of the renderer. The window remains
invisible to the user while DWM continues composing it.

After the first frame is drawn and swapped, the startup guard calls `DwmFlush`,
removes the cloak, flushes DWM again, and restores ordinary window transitions.
The guard also attempts cleanup if initialization exits early. These settings
apply to the individual window, not to the system's animation preferences.

The launcher calls `CreateProcessW` with `STARTF_USESHOWWINDOW` and `SW_HIDE`.
Neovide then controls when the window is exposed. This retains the hidden-start
condition of the tested workaround without starting a PowerShell interpreter.

## Validation and limits

The following runtime and visual validation was performed on the original Neovide 0.16.2 base and must be repeated for the rebased main build.

The original release build and an MSVC launcher build with warnings as errors passed.
A native Windows smoke check verified that the prepared window, submitted first
frame and revealed window had identical physical dimensions; the revealed window
was maximized, visible and no longer cloaked. Restore/maximize still worked.
Manual testing with OpenGL and maximized startup confirmed elimination of the
visible startup flicker using the hidden-start presentation workaround.

An isolated child-process test verified the native launcher's hidden startup
state, OpenGL argument and argument forwarding without opening an editor window.
The final native-launcher shortcut still requires a manual end-to-end comparison;
the earlier successful visual test used a PowerShell hidden-start launcher.

This remains an experimental workaround. The individual contributions of
cloaking, pre-maximization and temporary transition suppression have not been
isolated. D3D, non-maximized startup, fullscreen, other display/DPI arrangements,
and additional-window lifecycle paths have not been validated for this change.
No end-to-end startup benchmark is claimed.
