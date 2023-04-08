use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Mora {
    text: String,
    #[serde(default)]
    consonant: Option<String>,
    #[serde(default)]
    consonant_length: Option<f32>,
    vowel: String,
    vowel_length: f32,
    pitch: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccentPhrase {
    moras: Vec<Mora>,
    accent: i32,
    #[serde(default)]
    pause_mora: Option<Mora>,
    #[serde(default)]
    is_interrogative: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioQueryType {
    accent_phrases: Vec<AccentPhrase>,
    #[serde(rename = "speedScale")]
    speed_scale: f32,
    #[serde(rename = "pitchScale")]
    pitch_scale: f32,
    #[serde(rename = "intonationScale")]
    intonation_scale: f32,
    #[serde(rename = "volumeScale")]
    volume_scale: f32,
    #[serde(rename = "prePhonemeLength")]
    pre_phoneme_length: f32,
    #[serde(rename = "postPhonemeLength")]
    post_phoneme_length: f32,
    #[serde(rename = "outputSamplingRate")]
    output_sampling_rate: i32,
    #[serde(rename = "outputStereo")]
    output_stereo: bool,
    kana: String,
}