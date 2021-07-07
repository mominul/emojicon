# emojicon
Find Emoji by using Emoticons and GitHub's emoji names.

```rust
use emojicon::Emojicon;

fn main() {
    let emojicon = Emojicon::new();
    assert_eq!(emojicon.get_by_emoticon("B-)"), Some("😎"));
    assert_eq!(emojicon.get_by_name("cool").unwrap().collect::<Vec<_>>(), ["😎", "🆒"]);
}
```

## Acknowledgment
* [emoticon](https://github.com/wooorm/emoticon) by Titus Wormer
* [gemoji](https://github.com/github/gemoji) by Github
