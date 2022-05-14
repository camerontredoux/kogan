use crate::graphql::{self, Media};

#[tokio::main]
pub async fn make_request() -> Result<(), Box<dyn std::error::Error>> {
    use crate::graphql::anime_data::Variables;
    let data = graphql::perform_query(Variables { id: Some(140960) })
        .await?
        .data
        .ok_or("No data found")?
        .media;

    for item in data {
        let val = serde_json::to_value(item).unwrap();
        println!("{:#?}", val);
        let s: Media = serde_json::from_value(val).unwrap();
        println!("{:#?}", s);
    }

    Ok(())
}
