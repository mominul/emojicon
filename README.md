# emojicon
Convert emoticons `B-)` into Unicode emoji's 😎

```rust
use emojicon::Emojicon;

fn main() {
    let emojicon = Emojicon::new();
    assert_eq!(emojicon.get("B-)"), Some("😎"));
}
```

## Acknowledgment
* [emoticon](https://github.com/wooorm/emoticon) by Titus Wormer