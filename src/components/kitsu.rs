/// Generated by https://quicktype.io
extern crate serde_json;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Anime {
    /// List of all anime datum
    pub data: Vec<Datum>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Datum {
    /// Details for the current anime datum
    pub attributes: Attributes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// Synopsis of the anime series
    pub synopsis: Option<String>,
    /// List of titles for the anime (e.g. English, Japanese, etc.)
    pub titles: Titles,
    /// The canonical title of the anime as it appears on the site
    pub canonical_title: Option<String>,
    /// The abbreviated and alternative titles of the anime
    pub abbreviated_titles: Vec<String>,
    /// The average user score of the anime (out of 100)
    pub average_rating: Option<String>,
    /// Average rating frequency of the anime for each rating value
    pub rating_frequencies: HashMap<String, String>,
    /// Release date
    pub start_date: Option<String>,
    /// End date
    pub end_date: Option<String>,
    /// The next episode release date of the anime
    pub next_release: Option<serde_json::Value>,
    /// Rank of the anime's popularity compared to other anime
    pub popularity_rank: Option<i64>,
    /// Rank of the anime's score using the average rating compared to other anime
    pub rating_rank: Option<i64>,
    /// Age rating (e.g. 'R', 'PG-13', 'G', etc.)
    pub age_rating: Option<String>,
    /// Age rating guide (e.g. 17+ (violence & profanity))
    pub age_rating_guide: Option<String>,
    /// Subtype (e.g. movie, special, OVA, etc.)
    pub subtype: Option<String>,
    /// Current status (e.g. finished, current, tba, etc.)
    pub status: Option<String>,
    /// Poster image url
    pub poster_image: Option<PosterImage>,
    /// Total episode count
    pub episode_count: Option<i64>,
    /// Episode length in minutes
    pub episode_length: Option<i64>,
    /// Total length of all episodes
    pub total_length: Option<i64>,
    /// If the anime is nsfw
    pub nsfw: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PosterImage {
    /// The url of the smallest image
    pub tiny: Option<String>,
    /// The url of the largest image
    pub large: Option<String>,
    /// The url of the medium image
    pub small: Option<String>,
    /// The url of the original image
    pub original: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Titles {
    /// The english title for the anime
    pub en: Option<String>,
    /// The rōmaji title for the anime
    pub en_jp: Option<String>,
    /// The japanese title for the anime
    pub ja_jp: Option<String>,
}
