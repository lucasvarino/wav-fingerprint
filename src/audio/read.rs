use hound;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct WavData {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration_seconds: f32,
}

impl WavData {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();
        
        let samples: Vec<i16> = reader.samples().collect::<Result<Vec<i16>, _>>()?;
        
        let normalized_samples: Vec<f32> = samples
            .iter()
            .map(|&sample| sample as f32 / i16::MAX as f32)
            .collect();
        
        let duration_seconds = normalized_samples.len() as f32 / spec.sample_rate as f32 / spec.channels as f32;
        
        Ok(WavData {
            samples: normalized_samples,
            sample_rate: spec.sample_rate,
            channels: spec.channels,
            duration_seconds,
        })
    }
    
    pub fn get_info(&self) -> String {
        format!(
            "Sample Rate: {} Hz\nChannels: {}\nDuration: {:.2} seconds\nTotal Samples: {}",
            self.sample_rate,
            self.channels,
            self.duration_seconds,
            self.samples.len()
        )
    }
}