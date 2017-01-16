#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;


#[derive(Serialize, Deserialize, Debug)]
struct Line {
    timestamp: u64,
    line: String,
    file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lines {
    lines: Vec<Line>,
}

impl Lines {
    pub fn new() -> Lines {
        Lines { lines: vec![] }
    }

    pub fn add_line<S: Into<String>>(&mut self, line: S, file: S) {
        let new_line = Line {
            timestamp: 1464041337000,
            line: line.into(),
            file: file.into(),
        };

        self.lines.push(new_line);
    }
}
