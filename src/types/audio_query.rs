use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioQueryType {
    #[serde(rename = "speedScale")]
    speed_scale: i32,
    #[serde(rename = "pitchScale")]
    pitch_scale: i32,
    #[serde(rename = "intonationScale")]
    intonation_scale: i32,
    #[serde(rename = "volumeScale")]
    volume_scale: i32,
    #[serde(rename = "prePhonemeLength")]
    pre_phoneme_length: i32,
    #[serde(rename = "postPhonemeLength")]
    post_phoneme_length: i32,
    #[serde(rename = "outputSamplingRate")]
    output_sampling_rate: i32,
    #[serde(rename = "outputStereo")]
    output_stereo: bool,
    kana: String,
}
