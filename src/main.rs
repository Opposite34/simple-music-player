use std::thread::sleep;
use std::time::Duration;

use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, Sink};

use device_query::{DeviceQuery, DeviceState, Keycode};

const VOLUME_DELTA: f32 = 0.05;
const VOLUME_MIN: f32 = 0.0;
const VOLUME_MAX: f32 = 2.0;

const VOLUME_CTRL_SLEEP: Duration = Duration::from_millis(100);

fn main() -> std::io::Result<()>{
    let(_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    //your music path here
    let file = File::open("music/the_25th_hour.wav")?;
    let buf = BufReader::new(file);

    let source = Decoder::new(buf).unwrap();
    sink.append(source);
    
    let device_state = DeviceState::new();
    
    let mut vol = sink.volume();
    let mut vol_key_pressed = false;
    let mut space_hold = false;
    
    while !sink.empty() {
        let keys: Vec<Keycode> = device_state.get_keys();
        
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
        
        if keys.contains(&Keycode::Left) {
            vol -= VOLUME_DELTA;
            vol_key_pressed = true;
        }
        
        if keys.contains(&Keycode::Right) {
            vol += VOLUME_DELTA;
            vol_key_pressed = true;
        }
        
        if vol_key_pressed {
            vol_key_pressed = false;

            let vol_not_clamped = (vol * 100.0).round() / 100.0;
            vol = vol_not_clamped.clamp(VOLUME_MIN, VOLUME_MAX);
            
            if vol_not_clamped == vol {
                sink.set_volume(vol);
                
                println!("Current Volume: {:.2}", sink.volume());
                sleep(VOLUME_CTRL_SLEEP);
            }
        }
    }
    
    //block the ending of the current thread (just in case)
    sink.sleep_until_end();
    Ok(())
}
