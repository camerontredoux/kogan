#![allow(non_snake_case)]

use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct TrendingQuery;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Title {
    pub romaji: String,
    pub english: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CoverImage {
    pub medium: String,
    pub large: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct StartDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EndDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Edge {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Node {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct NextAiringEpisode {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trending {
    pub title: Title,
    pub description: String,
    pub coverImage: CoverImage,
    pub averageScore: f64,
    pub meanScore: f64,
    pub format: String,
    pub nextAiringEpisode: Option<NextAiringEpisode>,
    pub status: String,
    pub startDate: StartDate,
    pub endDate: EndDate,
    pub episodes: u32,
    pub duration: u32,
    pub seasonYear: u32,
    pub season: String,
}

/* pub async fn make_request(anime_name: &String) -> Result<Media, Box<dyn std::error::Error>> { */
/*     let data = TrendingQuery::perform_query(anime_data::Variables { */
/*         search: Some(String::from(anime_name)), */
/*     }) */
/*     .await? */
/*     .data */
/*     .ok_or("No data found")? */
/*     .media; */

/*     let data = serde_json::to_value(data).unwrap(); */
/*     let media: Trending = serde_json::from_value(data).unwrap(); */

/*     Ok(media) */
/* } */
