//! Convert emoticons `B-)` into Unicode emoji's 😎
//!
//! ```
//! use emojicon::Emojicon;
//! # fn main() {
//! 
//! let emojicon = Emojicon::new();
//! assert_eq!(emojicon.get("B-)"), Some("😎"));
//! # }
//! ```
//!
//! # Acknowledgment
//! * [emoticon](https://github.com/wooorm/emoticon) by Titus Wormer

use std::collections::HashMap;

mod emoticons;

pub struct Emojicon {
    emoticons: HashMap<&'static str, &'static str>,
}

impl Emojicon {
    /// Creates a new instance of `Emojicon`.
    pub fn new() -> Self {
        Self {
            emoticons: emoticons::emoticons()
        }
    }

    /// Gets the emoji for given `emoticon` wrapped in a `Option`.
    /// 
    /// ```
    /// # use emojicon::Emojicon;
    /// # fn main() {
    /// let emojicon = Emojicon::new();
    /// assert_eq!(emojicon.get(":-\"]"), Some("😊"));
    /// # }
    /// ```
    pub fn get(&self, emoticon: &str) -> Option<&str> {
        self.emoticons.get(emoticon).map(|i| *i)
    }
}

#[cfg(test)]
mod tests {
    use super::Emojicon;

    #[test]
    fn test() {
        let emoji = Emojicon::new();
        assert_eq!(emoji.get(":)").unwrap(), "😃");
        assert_eq!(emoji.get(":Z").unwrap(), "😒");
        assert_eq!(emoji.get(",:-[").unwrap(), "😓");
        assert_eq!(emoji.get("smile"), None);
    }
}
