use rand::thread_rng;
use rand::Rng;
use std::f32::consts::PI;
use std::i16;
use std::path::Path;

extern crate hound;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let path: &Path = "sine.wav".as_ref();

    let mut writer = match path.is_file() {
        true => hound::WavWriter::append(path).unwrap(),
        false => hound::WavWriter::create(path, spec).unwrap(),
    };

    // We should not append blindly, we should make sure that the existing file
    // has the right spec, because that is what we assume when writing.
    assert_eq!(spec, writer.spec());

    println!(
        "Old duration is {} seconds.",
        writer.duration() / spec.sample_rate
    );

    let amplitude = i16::MAX as f32;
    for _ in 1..10 {
        let freq = random_key();
        let duration = 1.0 * spec.sample_rate as f32;
        for t in (0..duration as u32).map(|x| x as f32 / spec.sample_rate as f32) {
            let sample = (t * freq * 2.0 * PI).sin();
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
    }

    println!(
        "New duration is {} seconds.",
        writer.duration() / spec.sample_rate
    );

    writer.finalize().unwrap();
}

// returns a random key frequency (using frequency_of_key) between 28 and 64 (inclusive)
fn random_key() -> f32 {
    let mut rng = thread_rng();
    let key = rng.gen_range(28..65);
    frequency_of_key(key)
}

// calculate the frequency in Hz of a given key
// key is a number from 1 to 88
// which corresponds to the note A0 (key1) to C8 (key88)
// example key 1 = 27.5, key 88 = 4186.009
fn frequency_of_key(key: u8) -> f32 {
    let freq = 440.0;
    let freq_of_key = freq * 2.0_f32.powf((key as f32 - 49.0) / 12.0);
    freq_of_key
}
