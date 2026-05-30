# mini-log

A tiny Rust logging crate controlled by the `LOG_LEVEL` environment variable.

Logging is disabled by default.

## Install

Add the crate to your project:

```bash
cargo add mini-log
```

## Usage

```rust
use mini_log::*;

fn main() {
    LogMessage::new(Level::Warning, "My Message here");
}
```

By default, this prints nothing.

To enable output, set the `LOG_LEVEL` environment variable.

```bash
LOG_LEVEL=WARNING cargo run
```

Example output:

```text
[2026-05-30 - 09:30:42:10] - WARNING - My Message here
```

## Log levels

```text
LOG_LEVEL=DEBUG    prints DEBUG, INFO, WARNING, ERROR
LOG_LEVEL=INFO     prints INFO, WARNING, ERROR
LOG_LEVEL=WARNING  prints WARNING, ERROR
LOG_LEVEL=WARN     prints WARNING, ERROR
LOG_LEVEL=ERROR    prints ERROR only
LOG_LEVEL=OFF      prints nothing
```

Example if `LOG_LEVEL` is missing or invalid:

```text
mini-log: LOG_LEVEL is not set, no logs will be printed.
Set one of these environment variable:
    LOG_LEVEL=DEBUG
    LOG_LEVEL=INFO
    LOG_LEVEL=WARNING
    LOG_LEVEL=ERROR
    LOG_LEVEL=OFF
```

## Windows

Powershell:
```powershell
$env:LOG_LEVEL="WARNING"
cargo run
```

cmd.exe
```cmd
set LOG_LEVEL=WARNING
cargo run
```

## macOS / Linux:

```bash
LOG_LEVEL=WARNING cargo run
```

Only this will be printed:

```text
[2026-05-30 - 09:30:42:10] - WARNING - Warning message
[2026-05-30 - 09:30:42:10] - ERROR - Error message
```

## Time format

By default, `mini-log` uses 24h timestamp format:

```text
2026-05-30 - 09:30:42:10 (YYYY-MM-dd - hour:minute:second:millisecond)
```

The full log output looks like this:

```text
[2026-05-30 - 09:30:42:10] - WARNING - My Message here
```

The timestamp format can be changed with the `LOG_TIME_FORMAT` environment variable.

Supported values:

```text
LOG_TIME_FORMAT=DEFAULT    2026-05-30 - 09:30:42:10
LOG_TIME_FORMAT=RFC3339    2026-05-30T09:30:42.100+02:00
LOG_TIME_FORMAT=TIME_ONLY  09:30:42
LOG_TIME_FORMAT=DATE_ONLY  2026-05-30
LOG_TIME_FORMAT=UNIX       1780126242
```

Example:

```bash
LOG_LEVEL=WARNING LOG_TIME_FORMAT=RFC3339 cargo run
```

Example output:

```text
[2026-05-30T09:30:42.100+02:00] - WARNING - My Message here
```

If `LOG_TIME_FORMAT` is missing or invalid, `mini-log` uses the default timestamp format.


## Examples

```rust
use mini_log::*;

fn main() {
    LogMessage::new(Level::Debug, "Debug message");
    LogMessage::new(Level::Info, "Info message");
    LogMessage::new(Level::Warning, "Warning message");
    LogMessage::new(Level::Error, "Error message");
}
```

```rust
use mini_log::*;

fn main() {
    if is_enabled(Level::Debug) {
        println!("Debug logging is currently enabled");
    }

    LogMessage::new(Level::Warning, "My Message here");
}
```



## Notes

`mini-log` writes to stdout using `println!`.

It is intentionally small and simple.
