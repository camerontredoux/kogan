#![allow(non_snake_case)]

use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Deserializer, Serialize};

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
pub struct Characters {
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
    pub pageInfo: PageInfo,
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
pub struct PageInfo {
    pub total: i32,
    pub perPage: i32,
    pub currentPage: i32,
    pub lastPage: i32,
    pub hasNextPage: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct NextAiringEpisode {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
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
    pub characters: Characters,
}

impl Media {
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
}
