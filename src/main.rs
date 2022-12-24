//sleeps
use std::thread::sleep;
use std::time::Duration;

//file path
use std::fs::File;
use std::path::Path;

//input
use std::io::stdin;
use std::io::BufReader;

//audio
use rodio::{Decoder, OutputStream, Sink};

//keyboard inputs
use device_query::{DeviceQuery, DeviceState, Keycode};

//for changing in small or large intervals
const VOLUME_DELTA_SMALL: f32 = 0.05;
const VOLUME_DELTA: f32 = 0.2;

const VOLUME_MIN: f32 = 0.0;
const VOLUME_MAX: f32 = 2.0;

const VOLUME_CTRL_SLEEP: Duration = Duration::from_millis(100);

fn main() -> std::io::Result<()>{
    let(_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink: Sink = Sink::try_new(&stream_handle).unwrap();
    
    //get user input for music path
    let mut input: String = String::new();

    print!("Please input path to music: ");
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    let music_path: &Path = Path::new(&input);

    let file: File = File::open(music_path)?;
    let buf: BufReader<File> = BufReader::new(file);

    let source: Decoder<BufReader<File>> = Decoder::new(buf).unwrap();
    sink.append(source);
    
    let device_state: DeviceState = DeviceState::new();
    
    let mut vol: f32 = sink.volume();
    let mut vol_key_pressed: bool = false;
    let mut space_hold: bool = false;
    
    while !sink.empty() {
        let keys: Vec<Keycode> = device_state.get_keys();
        
        // PAUSE/PLAY
        if keys.contains(&Keycode::Space) && !space_hold {
            space_hold = true;
            if sink.is_paused() {
                println!("Playing");
                sink.play();
            } 
            else {
                println!("Pausing");
                sink.pause();
            };
        }
        else if !keys.contains(&Keycode::Space) {
            space_hold = false;
        }

        //VOLUME CONTROL
        //Based on most keyboard layouts, may not work in some layout such as JIS
        if keys.contains(&Keycode::Up) || keys.contains(&Keycode::Minus) {
            if keys.contains(&Keycode::LShift) {
                vol += VOLUME_DELTA_SMALL;
            }
            else {
                vol += VOLUME_DELTA;
            }
            vol_key_pressed = true;
        }
        
        if keys.contains(&Keycode::Down) || keys.contains(&Keycode::Equal){
            if keys.contains(&Keycode::LShift) {
                vol -= VOLUME_DELTA_SMALL;
            }
            else {
                vol -= VOLUME_DELTA;
            }

            vol_key_pressed = true;
        }
        
        if vol_key_pressed {
            vol_key_pressed = false;

            let vol_not_clamped: f32 = (vol * 100.0).round() / 100.0;
            vol = vol_not_clamped.clamp(VOLUME_MIN, VOLUME_MAX);
            
            if vol_not_clamped == vol {
                sink.set_volume(vol);
                
                println!("Volume: {:.2}", sink.volume());
                sleep(VOLUME_CTRL_SLEEP);
            }
        }

        //STOP THE PLAYER
        if keys.contains(&Keycode::Escape) {
            sink.stop();
        }
    }
    
    //block the ending of the current thread (just in case)
    sink.sleep_until_end();

    println!("Exiting");
    Ok(())
}
