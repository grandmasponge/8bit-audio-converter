use minimp3::Decoder;
use minimp3::Error;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
fn main() -> Result<(), Error> {
    let file = File::open("./audio/whatislove.mp3").unwrap();
    let reader = BufReader::new(file);
    let mut decoder = Decoder::new(reader);
    let mut output: Vec<i8> = Vec::new();
    for frame in decoder.next_frame() {
        for mut samples in frame.data {
            let samples = samples / 256;
            let sample: i8 = samples.try_into().unwrap();
            output.push(sample);
        }
    }

    let newFile = File::create("./audio/8-bit.raw").unwrap();
    let mut writer = BufWriter::new(newFile);
    writer.write_all(output);

    Ok(())
}
