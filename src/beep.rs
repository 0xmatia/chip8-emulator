use sdl2::audio::{AudioCallback, AudioSpecDesired};

pub struct AudioDevice {
    device: sdl2::audio::AudioDevice<SquareWave>,
}

impl AudioDevice {
    pub fn new(context: &sdl2::Sdl) -> AudioDevice {
        let audio_subsystem = context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // Show obtained AudioSpec
                println!("{:?}", spec);

                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        // Start playback
        device.resume();
        AudioDevice {
            device
        }
    }
    
    pub fn beep(&self) {
        self.device.resume();
    }

    pub fn stop_beep(&self) {
        self.device.pause();
    }

}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
