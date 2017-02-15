extern crate whatlang;
use whatlang::{detect, Lang, Script};

fn main() {
    // Detect Esperanto (there are also `detect_lang` and `detect_script` functions)
    let info = detect("平仮名 一番 美しい").unwrap();
    println!("{:?}", info.lang);
    println!("{:?}", info.script);
}
