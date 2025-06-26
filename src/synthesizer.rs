use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::{AudioSubsystem, Sdl};

use std::time::{Instant, Duration};

const SAMPLE_RATE: i32 = 44100;

mod tetris_songs;

type SampleCounter = usize;

pub fn init(audio_subsystem: &AudioSubsystem) -> AudioDevice<Synthesizer> {
    let desired_spec = AudioSpecDesired {
        freq: Some(SAMPLE_RATE),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            Synthesizer {
                tracker: 0,
                // track: vec!({ Note {
                //     start_time : (SAMPLE_RATE as SampleCounter), 
                //     end_time: (SAMPLE_RATE as SampleCounter) * 5,
                //     volume: 1.,
                //     interval_length: 1000,
                // }}),
                track: tetris_songs::SONG_3.to_vec(),
            }
    }).unwrap();

    device.resume();

    device
}

#[derive(Clone)]
pub struct Note {
    start_time: SampleCounter,
    end_time: SampleCounter,
    volume: f32,
    interval_length: SampleCounter, //the number of audio samples per cycle
}

impl Note {
    fn sample(&self, tracker: &SampleCounter) -> f32 {
        // get an index within a single wave cycle
        let local_tracker: SampleCounter = (*tracker) % self.interval_length;

        if local_tracker > self.interval_length/2 {
            self.volume
        } else {
            -self.volume
        }
    }
}

pub struct Synthesizer {
    track: Vec<Note>,
    tracker: SampleCounter,
}

impl AudioCallback for Synthesizer {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            self.tracker += 1;
            *x = 0.;
            for note in &self.track {
                if note.start_time <= self.tracker && note.end_time >= self.tracker {
                    *x += note.sample(&self.tracker) / 4.;
                }
            }

        }
    }
}
