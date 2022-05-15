use crate::graphql::{anime_data, Media};

pub async fn make_request(anime_name: &String) -> Result<Media, Box<dyn std::error::Error>> {
    let data = Media::perform_query(anime_data::Variables {
        search: Some(String::from(anime_name)),
    })
    .await?
    .data
    .ok_or("No data found")?
    .media;

    let data = serde_json::to_value(data).unwrap();
    let media: Media = serde_json::from_value(data).unwrap();

    Ok(media)
}
