#[macro_use]
extern crate num_fields;

trait NumFields {
    fn num_fields() -> usize;
}

#[derive(NumFields)]
struct Hello {
    a: String,
    b: String,
}

fn main() {
    let n = Hello::num_fields();
    println!("{}", n);
}
