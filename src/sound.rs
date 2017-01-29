use futures::stream::Stream;
use futures::task::Executor;
use futures::executor;
use futures::task::Run;
use futures::Future;
use futures::sync::mpsc::Receiver;
use futures::stream::MergedItem;

use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::Arc;
use std::u16;
use std::i16;

use cpal;

struct MyExecutor;

impl Executor for MyExecutor {
  fn execute(&self, r: Run) {
    r.run();
  }
}

#[derive(Debug)]
pub enum SoundEvent {
  Beep,
  Boop
}

#[derive(Copy, Clone)]
pub enum Tone {
  Beep,
  Boop,
  Silent
}

#[derive(Clone)]
pub struct SoundState {
  pub is_playing: bool,
  pub timer: Option<(SystemTime, Duration)>,
  pub tone: Tone
}

impl SoundState {
  pub fn new() -> SoundState {
    SoundState {
      is_playing: false,
      timer: None,
      tone: Tone::Boop
    }
  }
}

pub fn spawn_audio_thread(sound_rx: Receiver<SoundEvent>) {
  thread::spawn(move || {
    Some(()).expect("Sneaky error");
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list().unwrap().next().expect("Failed to get endpoint format");
    let executor = Arc::new(MyExecutor);
    let event_loop = cpal::EventLoop::new();
    let samples_rate = format.samples_rate.0 as f32;
    let (mut voice, stream) = cpal::Voice::new(&endpoint, &format, &event_loop).expect("Failed to create a voice");
    voice.play();
    let mut data = (0u64..).map(move |t| t as f32 * 2.0 * 3.141592 / samples_rate);

    executor::spawn(stream.merge(sound_rx).fold(SoundState::new(), move |prev_state, item| -> Result<_, ()> {
      let mut next_state = prev_state.clone();

      next_state.timer = next_state.timer.and_then(|(start, duration_remaining)| {
        let now = SystemTime::now();
        let elapsed = now.duration_since(start).unwrap();
        duration_remaining.checked_sub(elapsed).map(|new_duration_remaining| {
          (now, new_duration_remaining)
        })
      });

      let mut on_buffer = |buffer, tone| {
        let tone_freq = match tone {
          Tone::Beep => 587.33,
          Tone::Boop => 440.0,
          Tone::Silent => 0.0
        };

        match buffer {
          cpal::UnknownTypeBuffer::U16(mut buffer) => {
            for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data) {
              let value = (value * tone_freq).sin();
              let value = ((value * 0.5 + 0.5) * u16::MAX as f32) as u16;
              for out in sample.iter_mut() { *out = value; }
            }
          },

          cpal::UnknownTypeBuffer::I16(mut buffer) => {
            for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data) {
              let value = (value * tone_freq).sin();
              let value = (value * i16::MAX as f32) as i16;
              for out in sample.iter_mut() { *out = value; }
            }
          },

          cpal::UnknownTypeBuffer::F32(mut buffer) => {
            for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data) {
              let value = (value * tone_freq).sin();
              for out in sample.iter_mut() { *out = value.sin(); }
            }
          },
        };
      };

      match item {
        MergedItem::First(buffer) => on_buffer(buffer, next_state.timer.map(|_| next_state.tone).unwrap_or(Tone::Silent)),
        MergedItem::Second(sound_event) => {
          next_state.timer = Some((SystemTime::now(), Duration::from_millis(100)));
          match sound_event {
            SoundEvent::Beep => next_state.tone = Tone::Beep,
            SoundEvent::Boop => next_state.tone = Tone::Boop,
          }
        },
        MergedItem::Both(buffer, sound_event) => {
          next_state.timer = Some((SystemTime::now(), Duration::from_millis(100)));
          match sound_event {
            SoundEvent::Beep => next_state.tone = Tone::Beep,
            SoundEvent::Boop => next_state.tone = Tone::Boop,
          }
          on_buffer(buffer, next_state.timer.map(|_| next_state.tone).unwrap_or(Tone::Silent));
        }
      };

      Ok(next_state)
    }).execute(executor);
    event_loop.run();
  });
}




