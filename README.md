# rsclip
CLI utility to copy the contents of a file to the clipboard written in Rust.
Inspired in xclip. Supports X11, _sometime Wayland_.

## Installation

### Cargo
```bash
cargo install rclip
```

## Usage
### Copy from pipe
Copy the contents of a pipe to the clipboard.

```bash
$ echo "Hello world" | rsclip
$ rsclip
Hello world
```


### Paste
Paste the contents of a file to the clipboard.

```bash
$ rclip
Clipboard content
$ rclip -p
Clipboard content
$ rclip --paste
Clipboard content
```

### Copy
Copy the contents of a file to the clipboard.

```bash
$ rclip <file>
$ rclip -c <file>
$ rclip --copy <file>
```
## todo
- [X] Copy from pipe
- [ ] Solve issues under Wayland
- [ ] Config files to change default behavior
- [ ] Add clipboard history
- [ ] Add colorful output
- [ ] Add clear flag
- [ ] Test
- [ ] Sanitize clipboard contect
- [ ] Migrate to clap for argument management
