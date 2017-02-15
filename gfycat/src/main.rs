extern crate gif;
extern crate png;

use std::fs::File;
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::borrow::Cow;


fn main() {

    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let (width, height) = (264, 262);
    let mut image = File::create("trump.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
    for n in 0..71 {
        let decoder = png::Decoder::new(File::open(format!("GIF/Trump/Trump_{:02}.png", n))
            .unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();
        encoder.set(Repeat::Infinite).unwrap();
        let frame = gif::Frame::from_rgb(width, height, &mut buf);
        encoder.write_frame(&frame).unwrap();
        sdfs
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
