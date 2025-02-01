#[derive(Debug, serde::Serialize)]
pub struct CobraHit {
    #[serde(rename = "perfect")]
    hit_300: i16,
    #[serde(rename = "good")]
    hit_100: i16,
    #[serde(rename = "bad")]
    hit_50: i16,
    #[serde(rename = "marvelous")]
    hit_geki: i16,
    #[serde(rename = "great")]
    hit_katu: i16,
    #[serde(rename = "miss")]
    hit_miss: i16,

    #[serde(skip)]
    slider_breaks: i16,
    #[serde(skip)]
    unstable_rate: f64,
    // TODO hitErrorArray
}
