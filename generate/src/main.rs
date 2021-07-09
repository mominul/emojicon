use serde::Deserialize;
use std::{collections::BTreeMap, env::var_os, fs::File, io::{Write, BufWriter}, path::PathBuf};
use std::fs::read;

#[derive(Deserialize)]
struct Emoticon {
    emoji: String,
    emoticons: Vec<String>,
}

#[derive(Deserialize)]
struct Emoji {
    emoji: String,
    aliases: Vec<String>,
    tags: Vec<String>,
}

fn create_emoticon_data() {
    let root = PathBuf::from(var_os("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.parent().unwrap();
    let dest = parent.join("src").join("emoticons.rs");

    let mut file = BufWriter::new(File::create(dest).unwrap());
    let source = read(parent.join("data/emoticon.json")).unwrap();
    let emoticons: Vec<Emoticon> = serde_json::from_slice(&source).unwrap();

    writeln!(&mut file, "// Do not edit this file manually! This was generated by the `generator` crate of this repository.").unwrap();
    writeln!(&mut file, "use std::collections::HashMap;\n").unwrap();
    writeln!(&mut file, "pub(crate) fn emoticons() -> HashMap<&'static str, &'static str> {{").unwrap();
    writeln!(&mut file, "\tvec![").unwrap();

    for Emoticon { emoji, emoticons } in emoticons {
        for icons in emoticons {
            writeln!(&mut file, "\t\t(\"{}\", \"{}\"),", icons.escape_debug(), emoji).unwrap();
        }
    }

    writeln!(&mut file, "\t]\n\t.into_iter()\n\t.collect()\n}}").unwrap();
}

fn create_emoji_data() {
    let root = PathBuf::from(var_os("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.parent().unwrap();
    let dest = parent.join("src").join("emojis.rs");

    let mut file = BufWriter::new(File::create(dest).unwrap());
    let source = read(parent.join("gemoji/db/emoji.json")).expect("Can't load ../gemoji/db/emoji.json. Try git submodule update --init");
    let emojis: Vec<Emoji> = serde_json::from_slice(&source).unwrap();
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for Emoji { emoji, aliases, tags } in emojis {
        for alias in aliases {
            map.entry(alias).or_insert(Vec::new()).push(emoji.to_owned());
        }

        for tag in tags {
            map.entry(tag).or_insert(Vec::new()).push(emoji.to_owned());
        }
    }

    writeln!(&mut file, "// Do not edit this file manually! This was generated by the `generator` crate of this repository.").unwrap();
    writeln!(&mut file, "use std::collections::HashMap;\n").unwrap();
    writeln!(&mut file, "pub(crate) fn emojis() -> HashMap<&'static str, Vec<&'static str>> {{").unwrap();
    writeln!(&mut file, "\tvec![").unwrap();

    for (text, emojis) in map {
        write!(&mut file, "\t\t(\"{}\", vec![", text).unwrap();
        for emoji in emojis {
            write!(&mut file, "\"{}\", ", emoji).unwrap();
        }
        write!(&mut file, "]),\n").unwrap();
    }

    writeln!(&mut file, "\t]\n\t.into_iter()\n\t.collect()\n}}").unwrap();
}

fn create_bengali_emoji_data() {
    let root = PathBuf::from(var_os("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.parent().unwrap();
    let dest = parent.join("src").join("bn_emojis.rs");

    let mut file = BufWriter::new(File::create(dest).unwrap());
    let source = read(parent.join("data/emoji-bn.json")).unwrap();
    let emojis: BTreeMap<String, String> = serde_json::from_slice(&source).unwrap();
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (emoji, code) in emojis {
        // Ignore entries which have spaces, we'll need to figure out what to do about them later.
        if code.contains(" ") {
            continue;
        }

        let codes: Vec<_> = code.split(',').collect();

        for code in codes {
            map.entry(code.to_owned()).or_insert(Vec::new()).push(emoji.to_owned());
        }
    }

    writeln!(&mut file, "// Do not edit this file manually! This was generated by the `generator` crate of this repository.").unwrap();
    writeln!(&mut file, "use std::collections::HashMap;\n").unwrap();
    writeln!(&mut file, "pub(crate) fn emojis() -> HashMap<&'static str, Vec<&'static str>> {{").unwrap();
    writeln!(&mut file, "\tvec![").unwrap();

    for (text, emojis) in map {
        write!(&mut file, "\t\t(\"{}\", vec![", text).unwrap();
        for emoji in emojis {
            write!(&mut file, "\"{}\", ", emoji).unwrap();
        }
        write!(&mut file, "]),\n").unwrap();
    }

    writeln!(&mut file, "\t]\n\t.into_iter()\n\t.collect()\n}}").unwrap();
}

fn main() {
    create_emoticon_data();
    create_emoji_data();
    create_bengali_emoji_data();
}
