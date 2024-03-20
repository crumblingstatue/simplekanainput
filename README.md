# simplekanainput

Simple kana input that copies its output to the clipboard.

Provides kanji suggestions as well as dictionary info.

![image](https://github.com/crumblingstatue/simplekanainput/assets/1521976/f9797f6c-9df2-4103-a2ce-8e9621d46f1c)

## Platform support

It is tested on Linux, and there is a web version at <https://crumblingstatue.github.io/simplekanainput/>.
Currently no one is testing it on Windows or Mac OS, so I can't promise anything about them.

## Building
There are two backends, `backend-sfml` and `backend-eframe`.
Currently, `backend-sfml` is the default, but it requires
[SFML](https://github.com/jeremyletang/rust-sfml?tab=readme-ov-file#requirements).
If you just want an easy to build version, try building with
```
cargo build --release --no-default-features --features=backend-eframe,ipc
```

## Usage

Set a shortcut key on your system to open simplekanainput, to have quick access to it.
To copy the output to the clipboard and close the window, press `ctrl+enter`.
simplekanainput will stay open in the background, waiting for an IPC message.
If you invoke simplekanainput with your shortcut key or otherwise, the window will open again.

### Help
See [Help.md](./Help.md).
