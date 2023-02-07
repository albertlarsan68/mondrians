use std::io::Cursor;

use mondrians::generate;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const SEP_WIDTH: u32 = 5;
const MAX_DEPTH: u32 = 5;
const MIN_MIDDLE: u32 = 20;
const MAX_MIDDLE: u32 = 80;

fn main() {
    let seed = "";
    let b64 = generate(
        seed.as_bytes(),
        WIDTH,
        HEIGHT,
        MAX_DEPTH,
        SEP_WIDTH,
        MIN_MIDDLE,
        MAX_MIDDLE,
    );

    std::io::copy(
        &mut base64::read::DecoderReader::new(
            Cursor::new(b64),
            &base64::engine::general_purpose::STANDARD,
        ),
        &mut std::fs::File::create("out/image.png").unwrap(),
    )
    .unwrap();
}
