# Simple Kana Input

## Basic usage
Type lowercase romaji into the text editor to convert it into japanese.
The default conversion mode is hiragana.
**ohayou** for example will be translated to **おはよう**.

## Suggestions
If you type in a word that gets recognized by simple kana input, it might suggest
applicable kanji. Press `Tab` and `Shift+Tab` to cycle through the available
suggestions.


### Deconjugation
Simple kana input will try to look for conjugation patterns, and if it finds a match
it will give you suggestions for the word, as well as info on how it's conjugated.
For example, if you type **jittoshiterarenai**, it will get recognized as **凝乎と**,
which is a **する** verb, and the conjugation is **て** form + **potential** + **ない**.

## Shortcut keys
key                  | effect                           | Note
---------------------|----------------------------------|-----------
F2                   | Copy output to clipboard         |
Ctrl + enter         | Copy output and hide window      | not on web
Tab                  | Select next kanji suggestion     |
Shift + tab          | Select previous kanji suggestion |
Alt + left           | Jump to previous word            |
Alt + right          | Jump to next word                |
Ctrl + left bracket  | Set style to hiragana            | `ctrl` + `[`
Ctrl + right bracket | Set Style to katakana            | `ctrl` + `]`

## Special characters
Certain characters have special use

- `{` and `}`   Delimit a "literal" segment that doesn't get converted to japanese
- `[` and `]`   Japanese quote marks 「 and 」
- `-`         Prolonged sound mark **ー**
- `.`         Japanese 。
- `,`         Japanese 、
- `!`         Japanese ！
- `?`         Japanese ？
- `...`       Japanese …
