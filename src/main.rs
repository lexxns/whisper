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

    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * note_to_freq("A") * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }

    println!(
        "New duration is {} seconds.",
        writer.duration() / spec.sample_rate
    );

    writer.finalize().unwrap();
}

// calculate the frequency in Hz of a given note
fn note_to_freq(note: &str) -> f32 {
    let mut freq = 440.0;
    for c in note.chars() {
        match c {
            'C' => freq *= 1.0,
            'D' => freq *= 2.0,
            'E' => freq *= 3.0,
            'F' => freq *= 4.0,
            'G' => freq *= 5.0,
            'A' => freq *= 6.0,
            'B' => freq *= 7.0,
            _ => panic!("Invalid note"),
        }
    }
    freq
}
