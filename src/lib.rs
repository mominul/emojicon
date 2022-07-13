//! Find Emoji by using Emoticons and GitHub's, Bengali emoji names.
//!
//! ```
//! use emojicon::{Emojicon, BengaliEmoji};
//! # fn main() {
//! 
//! let emojicon = Emojicon::new();
//! assert_eq!(emojicon.get_by_emoticon("B-)"), Some("😎"));
//! assert_eq!(emojicon.get_by_name("cool").unwrap().collect::<Vec<_>>(), ["😎", "🆒"]);
//! 
//! let emojis = BengaliEmoji::new();
//! assert_eq!(emojis.get("কুল").unwrap().collect::<Vec<_>>(), ["🆒", "😎"]);
//! # }
//! ```
//!
//! # Acknowledgment
//! * [emoticon](https://github.com/wooorm/emoticon) by Titus Wormer
//! * [gemoji](https://github.com/github/gemoji) by Github
//! * [bnemo](https://github.com/faruk-ahmad/bnemo) by Faruk Ahmad

use std::collections::HashMap;

mod emoticons;
mod emojis;
mod bn_emojis;

/// Find Emojis using Emoticons and GitHub's emoji names.
///
/// ```
/// # use emojicon::Emojicon;
/// # fn main() {
/// let emojicon = Emojicon::new();
/// assert_eq!(emojicon.get_by_emoticon("B-)"), Some("😎"));
/// assert_eq!(emojicon.get_by_name("cool").unwrap().collect::<Vec<_>>(), ["😎", "🆒"]);
/// # }
pub struct Emojicon {
    emoticons: HashMap<&'static str, &'static str>,
    emojis: HashMap<&'static str, &'static [&'static str]>,
}

impl Emojicon {
    /// Creates a new instance of `Emojicon`.
    pub fn new() -> Self {
        Self {
            emoticons: emoticons::emoticons(),
            emojis: emojis::emojis()
        }
    }

    /// Gets the emoji for given `emoticon` wrapped in a `Option`.
    /// 
    /// ```
    /// # use emojicon::Emojicon;
    /// # fn main() {
    /// let emojicon = Emojicon::new();
    /// assert_eq!(emojicon.get_by_emoticon(":-\"]"), Some("😊"));
    /// # }
    /// ```
    pub fn get_by_emoticon(&self, emoticon: &str) -> Option<&str> {
        self.emoticons.get(emoticon).map(|i| *i)
    }

    /// Get emojis by given `name`.
    ///
    /// ```
    /// # use emojicon::Emojicon;
    /// # fn main() {
    /// let emojicon = Emojicon::new();
    /// assert_eq!(emojicon.get_by_name("smile").unwrap().collect::<Vec<_>>(), ["😀", "😄"]);
    /// # }
    /// ```
    pub fn get_by_name(&self, name: &str) -> Option<impl Iterator<Item = &str>> {
        self.emojis.get(name).map(|v| v.iter().map(|s| *s))
    }
}

/// Find Emojis using Bengali(Bangla) names.
///
/// ```
/// # use emojicon::BengaliEmoji;
/// # fn main() {
/// let emojis = BengaliEmoji::new();
/// assert_eq!(emojis.get("হাসি").unwrap().collect::<Vec<_>>(), ["☺", "😀", "😁", "😃", "😄", "🙂"]);
/// assert_eq!(emojis.get("লল").unwrap().collect::<Vec<_>>(), ["😂", "🤣"]);
/// # }
pub struct BengaliEmoji {
    emojis: HashMap<&'static str, &'static [&'static str]>,
}

impl BengaliEmoji {
    /// Creates a new instance of `BengaliEmoji`.
    pub fn new() -> Self {
        Self { emojis: bn_emojis::emojis() } 
    }

    /// Get emojis by given `name`.
    ///
    /// ```
    /// # use emojicon::BengaliEmoji;
    /// # fn main() {
    /// let emojis = BengaliEmoji::new();
    /// assert_eq!(emojis.get("কুল").unwrap().collect::<Vec<_>>(), ["🆒", "😎"]);
    /// # }
    /// ```
    pub fn get(&self, name: &str) -> Option<impl Iterator<Item = &str>> {
        self.emojis.get(name).map(|v| v.iter().map(|s| *s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_emoticon() {
        let emoji = Emojicon::new();
        assert_eq!(emoji.get_by_emoticon(":)").unwrap(), "😃");
        assert_eq!(emoji.get_by_emoticon(":Z").unwrap(), "😒");
        assert_eq!(emoji.get_by_emoticon(",:-[").unwrap(), "😓");
        assert_eq!(emoji.get_by_emoticon("smile"), None);
    }

    #[test]
    fn test_by_name() {
        let emoji = Emojicon::new();
        assert_eq!(emoji.get_by_name("happy").unwrap().collect::<Vec<_>>(), ["😀", "😃", "😄", "😆"]);
        assert!(!emoji.get_by_name("none").is_some());
    }

    #[test]
    fn test_bengali_emoji() {
        let emojis = BengaliEmoji::new();
        assert_eq!(emojis.get("কষ্ট").unwrap().collect::<Vec<_>>(), ["😣"]);
        assert_eq!(emojis.get("নিরাশ").unwrap().collect::<Vec<_>>(), ["😑", "😔", "😦", "🙁"]);
    }
}
