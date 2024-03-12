# simplekanainput

Simple kana input that copies its output to the clipboard.

Provides kanji suggestions as well as dictionary info.

## Platform support

Right now it only supports Linux (due to the IPC mechanism using `/dev/shmem`), but contributions are welcome.

## Usage

Set a shortcut key on your system to open simplekanainput, to have quick access to it.
To copy the output to the clipboard and close the window, press `ctrl+enter`.
simplekanainput will stay open in the background, waiting for an IPC message.
If you invoke simplekanainput with your shortcut key or otherwise, the window will open again.

### Shortcut keys
| Key                  | Function                                              |
| -------------------- | ----------------------------------------------------- |
| `Ctrl+enter`         | Copy japanese to clipboard + close window             |
| `Esc`                | Close without copying anything                        |
| `Tab/Shift+tab`      | Cycle between kanji suggestions for the selected word |
| `Alt+left/Alt+right` | Move word selection cursor (highlighted word)         |
