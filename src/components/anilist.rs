#![allow(non_snake_case)]

use chrono::{DateTime, TimeZone};
use graphql_client::GraphQLQuery;
use sanitize_html::rules::predefined::DEFAULT;
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug, Serialize"
)]
pub struct TrendingQuery;

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub media: Vec<Media>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    idMal: Option<u32>,
    title: Title,
    description: Option<String>,
    coverImage: CoverImage,
    averageScore: Option<f64>,
    format: Option<String>,
    status: Option<String>,
    startDate: StartDate,
    endDate: EndDate,
    genres: Vec<String>,
    episodes: Option<u32>,
    duration: Option<u32>,
    seasonYear: Option<u32>,
    season: Option<String>,
    rankings: Vec<Ranking>,
    nextAiringEpisode: Option<NextAiringEpisode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NextAiringEpisode {
    airingAt: u32,
    timeUntilAiring: u32,
    episode: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ranking {
    rank: Option<u32>,
    #[serde(rename = "type")]
    kind: Option<String>,
    allTime: Option<bool>,
    context: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    romaji: Option<String>,
    english: Option<String>,
    native: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoverImage {
    medium: Option<String>,
    large: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartDate {
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndDate {
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
}

impl Media {
    pub fn id(&self) -> u32 {
        self.idMal.unwrap_or(0)
    }

    pub fn rankings(&self) -> String {
        let rankings = self
            .rankings
            .iter()
            .find(|r| r.kind.as_ref().unwrap() == "RATED");

        let (rank, context) = match rankings {
            Some(ranking) => match ranking.rank {
                Some(rank) => match &ranking.context {
                    Some(context) => (rank, context),
                    None => {
                        return String::from("Unrated");
                    }
                },
                None => return String::from("Unrated"),
            },
            None => return String::from("Unrated"),
        };

        format!("#{} {}", rank, context)
    }

    pub fn popularity(&self) -> String {
        let rankings = self
            .rankings
            .iter()
            .find(|r| r.kind.as_ref().unwrap() == "POPULAR");

        let (rank, context) = match rankings {
            Some(ranking) => match ranking.rank {
                Some(rank) => match &ranking.context {
                    Some(context) => (rank, context),
                    None => {
                        return String::from("Unrated");
                    }
                },
                None => return String::from("Unrated"),
            },
            None => return String::from("Unrated"),
        };

        format!("#{} {}", rank, context)
    }

    pub fn title_english(&self) -> String {
        match &self.title.english {
            Some(title) => title.to_owned(),
            None => "No title found".to_string(),
        }
    }

    pub fn title_romaji(&self) -> String {
        match &self.title.romaji {
            Some(title) => title.to_owned(),
            None => "No title found".to_string(),
        }
    }

    pub fn description(&self) -> String {
        match &self.description {
            Some(description) => sanitize_html::sanitize_str(&DEFAULT, description)
                .unwrap_or("No description provided.".to_owned()),
            None => "No description provided.".to_owned(),
        }
    }

    pub fn cover_image(&self) -> String {
        match &self.coverImage.large {
            Some(image) => image.to_owned(),
            None => "No image found".to_owned(),
        }
    }

    pub fn average_score(&self) -> &f64 {
        self.averageScore.as_ref().unwrap_or(&0.0)
    }

    pub fn format(&self) -> String {
        match &self.format {
            Some(format) => format.to_owned(),
            None => "No format provided.".to_owned(),
        }
    }

    pub fn status(&self) -> String {
        match &self.status {
            Some(status) => status.to_owned(),
            None => "No status provided.".to_owned(),
        }
    }

    pub fn start_date(&self) -> String {
        let year = self.startDate.year;
        let month = self.startDate.month;
        let day = self.startDate.day;

        match (year, month, day) {
            (Some(year), Some(month), Some(day)) => format!("{}-{}-{}", year, month, day),
            _ => "No start date provided.".to_owned(),
        }
    }

    pub fn end_date(&self) -> String {
        let year = self.endDate.year;
        let month = self.endDate.month;
        let day = self.endDate.day;

        match (year, month, day) {
            (Some(year), Some(month), Some(day)) => format!("{}-{}-{}", year, month, day),
            _ => "No end date provided.".to_owned(),
        }
    }

    pub fn genres(&self) -> String {
        self.genres.join(", ")
    }

    pub fn episodes(&self) -> &u32 {
        self.episodes.as_ref().unwrap_or(&0)
    }

    pub fn duration(&self) -> &u32 {
        self.duration.as_ref().unwrap_or(&0)
    }

    pub fn season_year(&self) -> &u32 {
        self.seasonYear.as_ref().unwrap_or(&0)
    }

    pub fn season(&self) -> String {
        match &self.season {
            Some(season) => season.to_owned(),
            None => "No season provided.".to_owned(),
        }
    }

    pub fn airingAt(&self) -> String {
        match &self.nextAiringEpisode {
            Some(episode) => {
                let dt = chrono::Local.timestamp(episode.airingAt as i64, 0);
                dt.format("%Y-%m-%d %I:%M:%S %p").to_string()
            }
            None => String::from("No release date"),
        }
    }

    pub fn timeUntilAiring(&self) -> String {
        match &self.nextAiringEpisode {
            Some(episode) => {
                let total_seconds = episode.timeUntilAiring;
                let days = total_seconds / 86400;
                let hours = (total_seconds - days * 86400) / 3600;
                let minutes = (total_seconds - days * 86400 - hours * 3600) / 60;
                let seconds = total_seconds - days * 86400 - hours * 3600 - minutes * 60;
                format!("{}D {}H {}M {}S", days, hours, minutes, seconds)
            }
            None => String::from("No release date"),
        }
    }

    pub fn episode(&self) -> &u32 {
        match &self.nextAiringEpisode {
            Some(episode) => &episode.episode,
            None => &0,
        }
    }
}
