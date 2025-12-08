use crate::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, source::Source};
use slab::Slab;
use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
    time::{Duration, Instant},
};

#[derive(Default, Debug, Clone)]
pub struct AudioCollection {
    pub audio: Vec<String>,
}

impl AudioCollection {
    pub fn new() -> Self {
        let entries = match fs::read_dir("./audio") {
            Ok(data) => data,
            Err(_) => return AudioCollection::default(),
        };

        let mut audio = Vec::with_capacity(32);
        audio.push("None".to_string());

        entries.for_each(|entry| {
            if let Ok(entry_data) = entry {
                let file_name = entry_data.file_name();
                let file_name_str = file_name.to_string_lossy().into_owned();
                audio.push(file_name_str);
            }
        });

        AudioCollection { audio }
    }
}

pub struct Audio {
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    music: Sink,
    weather: Sink,
    effects: Slab<Sink>,
}

impl Audio {
    pub fn new(volume: f32) -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let music = Sink::try_new(&stream_handle)?;
        music.set_volume(volume);

        let weather = Sink::try_new(&stream_handle)?;
        weather.set_volume(volume);

        Ok(Self {
            stream_handle,
            _stream: stream,
            music,
            weather,
            effects: Slab::new(),
        })
    }

    pub fn set_music(&mut self, source: impl AsRef<Path>) -> Result<()> {
        let file = BufReader::new(File::open(source)?);
        let source = Decoder::new(file)?;

        self.music.append(
            source
                .fade_in(Duration::from_secs(1))
                .repeat_infinite()
                .skippable(),
        );

        if self.music.len() > 1 {
            self.music.skip_one()
        }

        self.music.play();
        Ok(())
    }

    pub fn stop_music(&mut self) {
        self.music.clear();
    }

    pub fn set_weather(&mut self, source: impl AsRef<Path>) -> Result<()> {
        let file = BufReader::new(File::open(source)?);
        let source = Decoder::new(file)?;

        self.weather.append(
            source
                .fade_in(Duration::from_secs(1))
                .repeat_infinite()
                .skippable(),
        );

        if self.weather.len() > 1 {
            self.weather.skip_one()
        }

        self.weather.play();
        Ok(())
    }

    pub fn stop_weather(&mut self) {
        self.weather.clear();
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music.set_volume(volume);
        self.weather.set_volume(volume);
    }

    pub fn play_effect(&mut self, source: impl AsRef<Path>, volume: f32) -> Result<()> {
        let s_volume = volume * 0.01;
        let sink = Sink::try_new(&self.stream_handle)?;
        let file = BufReader::new(File::open(source)?);
        let source = Decoder::new(file)?;
        sink.set_volume(s_volume);
        sink.append(source);
        sink.play();
        self.effects.insert(sink);
        Ok(())
    }

    pub fn set_effect_volume(&mut self, volume: f32) {
        for effect in &mut self.effects {
            effect.1.set_volume(volume);
        }
    }

    pub fn update_effects(&mut self) {
        let mut rem = Vec::new();

        for (id, effect) in &self.effects {
            if effect.len() == 0 {
                rem.push(id);
            }
        }

        for id in rem {
            self.effects.remove(id);
        }
    }
}
