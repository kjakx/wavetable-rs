use std::f64::consts::PI;
use paste::paste;

//
// waveform functions
//

fn sine(phase: f64) -> f64 {
    (2.0 * PI * phase).sin()
}

fn tri(phase: f64) -> f64 {
    if phase < 0.5 {
        4.0 * phase - 1.0
    } else {
        -4.0 * phase + 3.0
    }
}

fn saw(phase: f64) -> f64 {
    -2.0 * phase + 1.0
}

fn square(phase: f64) -> f64 {
    if phase < 0.5 {
        1.0
    } else {
        -1.0
    }
}

//
// wavetable definition
//

pub trait WaveTable {
    fn new(size: usize) -> Self;
    fn synth(&self, n: usize, f: f64, fs: f64) -> f64;
}

macro_rules! impl_wavetable {
    ($($waveform:ident)+) => {
        $(
            pub struct $waveform {
                size: usize,
                table: Vec<f64>,
            }

            impl WaveTable for $waveform {
                fn new(size: usize) -> Self {
                    paste! {
                        let table: Vec<f64> = (0..size).map(|i| [<$waveform:lower>](i as f64 / size as f64)).collect();
                        $waveform { size, table }
                    }
                }

                fn synth(&self, n: usize, f: f64, fs: f64) -> f64 {
                    let pos = (n as f64 * f / fs).fract() * self.size as f64;
                    let rel_pos = pos / self.size as f64;
                    (1.0 - rel_pos) * self.table[pos as usize] + rel_pos * self.table[(pos as usize + 1) % self.size]
                }
            }
        )*
    }
}

impl_wavetable!{ Sine Tri Saw Square }

#[cfg(test)]
mod tests {
    use super::*;
    use hound;
    use dasp_sample::Sample;

    fn write_wave(wt: &impl WaveTable, f: f64, fs: f64, dur_sec: f64, name: &str) {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: fs as u32,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(name, spec).unwrap();
        for i in 0..(44100.0*dur_sec) as usize {
            writer.write_sample(wt.synth(i, f, fs).to_sample::<i16>()).unwrap();
        }
    }

    #[test]
    fn generate_sine_wave() {
        let wt = Sine::new(1024);
        write_wave(&wt, 531.33, 44100.0, 1.0, "wav/sine_wavetable_C.wav");
    }

    #[test]
    fn generate_tri_wave() {
        let wt = Tri::new(1024);
        write_wave(&wt, 531.33, 44100.0, 1.0, "wav/tri_wavetable_C.wav");
    }

    #[test]
    fn generate_saw_wave() {
        let wt = Saw::new(1024);
        write_wave(&wt, 531.33, 44100.0, 1.0, "wav/saw_wavetable_C.wav");
    }

    #[test]
    fn generate_square_wave() {
        let wt = Square::new(1024);
        write_wave(&wt, 531.33, 44100.0, 1.0, "wav/square_wavetable_C.wav");
    }
}
