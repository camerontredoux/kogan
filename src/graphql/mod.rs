#![allow(non_snake_case)]

use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};

pub mod anilist;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct AnimeData;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Title {
    romaji: String,
    english: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CoverImage {
    medium: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct StartDate {
    year: i32,
    month: i32,
    day: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EndDate {
    year: i32,
    month: i32,
    day: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Characters {
    edges: Vec<Edge>,
    nodes: Vec<Node>,
    pageInfo: PageInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Edge {
    id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Node {
    id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PageInfo {
    total: i32,
    perPage: i32,
    currentPage: i32,
    lastPage: i32,
    hasNextPage: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct NextAiringEpisode {
    id: i32,
}

pub async fn perform_query(
    variables: anime_data::Variables,
) -> Result<Response<anime_data::ResponseData>, Box<dyn std::error::Error>> {
    let request = AnimeData::build_query(variables);
    let client = reqwest::Client::new();

    let response = client
        .post("https://graphql.anilist.co/")
        .json(&request)
        .send()
        .await?;

    let response_body: Response<anime_data::ResponseData> = response.json().await?;

    Ok(response_body)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    pub title: Title,
    pub description: String,
    pub coverImage: CoverImage,
    pub averageScore: f64,
    pub meanScore: f64,
    pub format: String,
    pub nextAiringEpisode: NextAiringEpisode,
    pub status: String,
    pub startDate: StartDate,
    pub endDate: EndDate,
    pub episodes: u32,
    pub duration: u32,
    pub seasonYear: u32,
    pub season: String,
    pub characters: Characters,
}
