# emojicon
![crates.io](https://img.shields.io/crates/v/emojicon.svg)
![docs.rs](https://docs.rs/emojicon/badge.svg)

Find Emoji by using Emoticons and GitHub's, Bengali emoji names.

```rust
use emojicon::{Emojicon, BengaliEmoji};
fn main() {
    let emojicon = Emojicon::new();
    assert_eq!(emojicon.get_by_emoticon("B-)"), Some("ð"));
    assert_eq!(emojicon.get_by_name("cool").unwrap().collect::<Vec<_>>(), ["ð", "ð"]);
    
    let emojis = BengaliEmoji::new();
    assert_eq!(emojis.get("āĶā§āĶē").unwrap().collect::<Vec<_>>(), ["ð", "ð"]);
}
```

## Acknowledgment
* [emoticon](https://github.com/wooorm/emoticon) by Titus Wormer
* [gemoji](https://github.com/github/gemoji) by Github
* [bnemo](https://github.com/faruk-ahmad/bnemo) by Faruk Ahmad
