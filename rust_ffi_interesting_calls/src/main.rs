fn main() {
    let result = unsafe { MessageBeep(0x00000010) };

    match result {
        true =>  println!("working");
        false => println!("doesn't work"); 
    }
}   

#[link(name = "user32")]
unsafe extern "system" {
    // Plays a waveform sound. The waveform sound for each sound type is identified by an entry in the registry.
    fn MessageBeep(uType: u32) -> bool;
}