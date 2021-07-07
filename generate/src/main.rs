use serde::Deserialize;
use std::{env::var_os, fs::File, io::{Write, BufWriter}, path::PathBuf};
use std::fs::read;

#[derive(Deserialize, Debug)]
struct Emoticon {
    emoji: String,
    emoticons: Vec<String>,
}

fn create_emoticon_data() {
    let root = PathBuf::from(var_os("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.parent().unwrap();
    let dest = parent.join("src").join("emoticons.rs");

    let mut file = BufWriter::new(File::create(dest).unwrap());
    let source = read(parent.join("data/emoticon.json")).unwrap();
    let emoticons: Vec<Emoticon> = serde_json::from_slice(&source).unwrap();

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

fn main() {
    create_emoticon_data();
}
