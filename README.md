# i3-wk
This tool emulates the workspace behavior of xmonad and herbstluftwm. When switching to a workspace with this tool, it tries to pull the target workspace to the current screen. If the target workspace is visible on a different screen, `i3-wk` exchanges the workspace of the current screen with the workspace of the other screen.

## Usage
Call `i3-wk <workspace name>` and it switches to that workspace. If you want to replace the default workspace switch keys with `i3-wk` you can use these bindings:

	bindsym --release $mod+1 exec --no-startup-id i3-wk 1
	bindsym --release $mod+2 exec --no-startup-id i3-wk 2
	bindsym --release $mod+3 exec --no-startup-id i3-wk 3
	bindsym --release $mod+4 exec --no-startup-id i3-wk 4
	...

Using `--release` is necessary because i3 acts weird if `i3-wk` switches workspaces while the keys are still pressed.

## Runtime Dependencies
 * i3-msg

## Building

`i3-wk` is written in Rust and requires Cargo to download the dependencies.

	cargo build --release

The final binary is at `target/release/i3-wk` and can be copied or symlinked somewhere else if needed.