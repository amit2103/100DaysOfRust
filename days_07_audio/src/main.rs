extern crate rodio;

use std::fs;
use std::io::BufReader;

fn main() {
    let files = fs::read_dir("/home/arav/Downloads/music").unwrap();
    files
        .filter_map(Result::ok)
        .filter(|d| {
            if let Some(e) = d.path().extension() {
                e == "mp3"
            } else {
                false
            }
        })
        .for_each(|f| {
            let device = rodio::default_output_device().unwrap();
            let sink = rodio::Sink::new(&device);

            println!("{:?}", f);
            let file = std::fs::File::open(f.path()).unwrap();
            sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
           // sink.set_volume(1.5);

            sink.sleep_until_end();
        });
}
