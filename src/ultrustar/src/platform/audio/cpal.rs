use std::sync::{atomic::AtomicBool, Arc};

use super::Note;
use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use log::{error, info};
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug)]
pub struct HostId(cpal::HostId);
impl Default for HostId {
    fn default() -> Self {
        let hosts = ::cpal::available_hosts();
        let host = *hosts.first().expect("No available hosts");
        Self(host)
    }
}
impl Serialize for HostId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.name())
    }
}
impl<'de> Deserialize<'de> for HostId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostIdVisitor;

        impl<'de> Visitor<'de> for HostIdVisitor {
            type Value = HostId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                let names = cpal::available_hosts()
                    .iter()
                    .map(cpal::HostId::name)
                    .collect::<Vec<_>>();
                formatter.write_str(&format!("One of {:?}", names))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                cpal::available_hosts()
                    .iter()
                    .copied()
                    .find_map(|host| {
                        if host.name() == value {
                            Some(HostId(host))
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| serde::de::Error::invalid_value(Unexpected::Str(value), &self))
            }
        }

        deserializer.deserialize_any(HostIdVisitor)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct InitSettings {
    host: HostId,
}
impl crate::SettingsTrait for InitSettings {}

#[derive(Debug)]
enum SampleBuf {
    U16(Vec<u16>),
    I16(Vec<i16>),
    F32(Vec<f32>),
}
impl SampleBuf {
    fn from_sample_format(format: cpal::SampleFormat) -> Self {
        match format {
            cpal::SampleFormat::I16 => Self::I16(Vec::new()),
            cpal::SampleFormat::U16 => Self::U16(Vec::new()),
            cpal::SampleFormat::F32 => Self::F32(Vec::new()),
        }
    }
}
trait SampleBufSource {
    fn copy_to(&self, buf: &mut SampleBuf);
}
impl SampleBufSource for [i16] {
    fn copy_to(&self, buf: &mut SampleBuf) {
        if let SampleBuf::I16(buf) = buf {
            buf.clear();
            buf.extend_from_slice(self);
        } else {
            panic!("Using wrong buffer type");
        }
    }
}
impl SampleBufSource for [u16] {
    fn copy_to(&self, buf: &mut SampleBuf) {
        if let SampleBuf::U16(buf) = buf {
            buf.clear();
            buf.extend_from_slice(self);
        } else {
            panic!("Using wrong buffer type");
        }
    }
}
impl SampleBufSource for [f32] {
    fn copy_to(&self, buf: &mut SampleBuf) {
        if let SampleBuf::F32(buf) = buf {
            buf.clear();
            buf.extend_from_slice(self);
        } else {
            panic!("Using wrong buffer type");
        }
    }
}

#[allow(clippy::type_repetition_in_bounds)] // FIXME wtf?
fn make_stream_callback<T>(
    samples_in: crossbeam_channel::Sender<SampleBuf>,
    samples_out: crossbeam_channel::Receiver<SampleBuf>,
) -> impl FnMut(&[T], &cpal::InputCallbackInfo)
where
    T: cpal::Sample,
    [T]: SampleBufSource,
{
    move |samples: &[T], _info| {
        let mut buf = samples_out
            .try_recv()
            .expect("Input stream should always be able to write");
        samples.copy_to(&mut buf);
        samples_in
            .send(buf)
            .expect("Audio stream should outlive receiver");
    }
}

pub struct NoteInput {
    _dev: cpal::Device,
    _stream: cpal::Stream,
    samples_in: crossbeam_channel::Sender<SampleBuf>,
    samples_out: crossbeam_channel::Receiver<SampleBuf>,
    stream_err: Arc<AtomicBool>,
}
impl NoteInput {
    fn from_device(dev: cpal::Device) -> Result<Self> {
        let cfg = dev.default_input_config()?;
        let (samples_in, samples_out) = crossbeam_channel::bounded::<SampleBuf>(2);
        for _ in 0..2 {
            samples_in.send(SampleBuf::from_sample_format(cfg.sample_format()))?;
        }
        let stream_err = Arc::new(AtomicBool::new(false));
        let stream = {
            let samples_in = samples_in.clone();
            let samples_out = samples_out.clone();
            let stream_err = stream_err.clone();
            let err_callback = move |err| {
                error!("{}", err);
                stream_err.swap(true, std::sync::atomic::Ordering::Release);
            };
            match cfg.sample_format() {
                cpal::SampleFormat::I16 => dev.build_input_stream(
                    &cfg.into(),
                    make_stream_callback::<i16>(samples_in, samples_out),
                    err_callback,
                )?,
                cpal::SampleFormat::U16 => dev.build_input_stream(
                    &cfg.into(),
                    make_stream_callback::<u16>(samples_in, samples_out),
                    err_callback,
                )?,
                cpal::SampleFormat::F32 => dev.build_input_stream(
                    &cfg.into(),
                    make_stream_callback::<f32>(samples_in, samples_out),
                    err_callback,
                )?,
            }
        };
        stream.play()?;
        Ok(Self {
            _dev: dev,
            _stream: stream,
            samples_in,
            samples_out,
            stream_err,
        })
    }
    fn from_id(dev_id: &DeviceId, host: &cpal::Host) -> Result<Self> {
        let dev = host
            .input_devices()?
            .find_map(|dev| {
                if dev.name().ok()? == dev_id.0 {
                    Some(dev)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("Failed to lookup device"))?;
        Self::from_device(dev)
    }
}
impl super::NoteInput for NoteInput {
    fn read_current(&self) -> Result<Option<Note>> {
        if self.stream_err.load(std::sync::atomic::Ordering::Acquire) {
            return Err(anyhow!("Stream encountered error"));
        }
        let samples = match self.samples_out.try_recv() {
            Ok(samples) => samples,
            Err(_) => return Ok(None), //
        };
        info!("Read samples: {:?}", samples);
        self.samples_in.send(samples)?;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct DeviceId(String);

pub struct Platform {
    host: cpal::Host,
}
impl Platform {
    /// Create a `NoteInput` from the host's default input device
    ///
    /// # Errors
    ///
    /// If the platform's default input device cannot be opened
    pub fn default_note_input(&self) -> Result<NoteInput> {
        let dev = self
            .host
            .default_input_device()
            .ok_or_else(|| anyhow!("No default input device"))?;
        info!("Default device: {}", dev.name()?);
        NoteInput::from_device(dev)
    }
}

impl super::PlatformApi for Platform {
    type InitSettings = InitSettings;

    type NoteInputId = DeviceId;

    type NoteInput = NoteInput;

    fn init(settings: &Self::InitSettings) -> Result<Self> {
        info!("Initializing audio backend {}", settings.host.0.name());
        Ok(Self {
            host: cpal::host_from_id(settings.host.0)?,
        })
    }

    fn list_note_inputs(&self) -> Vec<Self::NoteInputId> {
        let inputs = match self.host.input_devices() {
            Ok(devs) => devs,
            Err(_) => return vec![],
        };
        inputs
            .filter_map(|input| -> Option<Self::NoteInputId> { Some(DeviceId(input.name().ok()?)) })
            .collect::<Vec<_>>()
    }

    fn create_note_input(&self, id: &Self::NoteInputId) -> Result<Self::NoteInput> {
        NoteInput::from_id(id, &self.host)
    }
}

#[cfg(test)]
mod test {
    use crate::platform::audio::PlatformApi;

    use super::{InitSettings, Platform};

    #[test]
    fn init() {
        let _platform = Platform::init(&InitSettings::default());
    }
}
