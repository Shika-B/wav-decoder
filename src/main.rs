mod utility;
mod wave_objects;
use wave_objects::Wave;

fn main() {
    let file = include_bytes!("sound.wav");
    let wave = Wave::new(file);
    println!("{:#?}", wave.header);
    println!("all good")
}
