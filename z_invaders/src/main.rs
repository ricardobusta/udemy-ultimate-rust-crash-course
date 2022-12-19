use std::error::Error;
use std::ptr::null;
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explode", "assets\\explode.wav");
    audio.add("lose", "assets\\lose.wav");
    audio.add("move", "assets\\move.wav");
    audio.add("pew", "assets\\pew.wav");
    audio.add("startup", "z_invaders/assets/startup.wav");
    audio.add("win", "assets\\win.wav");
    audio.play("startup");

    //cleanup
    audio.wait();

    Ok(())
}
