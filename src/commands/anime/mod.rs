pub mod announce;
pub mod info;
pub mod trending;

pub struct Anime<'a> {
    name: &'a str,
    image_url: &'a str,
    description: &'a str,
    rating: &'a str,
    episodes: i64,
    episode_length: i64,
    start_date: &'a str,
    end_date: &'a str,
    status: &'a str,
    age_rating: &'a str,
    age_rating_guide: &'a str,
}

impl<'a> Anime<'a> {
    pub fn new(anime_json: &'a serde_json::Value, index: usize) -> Self {
        let name = anime_json["data"][index]["attributes"]["canonicalTitle"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let description = anime_json["data"][index]["attributes"]["synopsis"]
            .as_str()
            .unwrap_or_else(|| "No description available");
        let image_url = anime_json["data"][index]["attributes"]["posterImage"]["small"]
            .as_str()
            .unwrap_or_else(|| "No image available");
        let rating = anime_json["data"][index]["attributes"]["averageRating"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let episodes = anime_json["data"][index]["attributes"]["episodeCount"]
            .as_i64()
            .unwrap_or_else(|| 0);
        let start_date = anime_json["data"][index]["attributes"]["startDate"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let end_date = anime_json["data"][index]["attributes"]["endDate"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let status = anime_json["data"][index]["attributes"]["status"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let episode_length = anime_json["data"][index]["attributes"]["episodeLength"]
            .as_i64()
            .unwrap_or_else(|| 0);
        let age_rating = anime_json["data"][index]["attributes"]["ageRating"]
            .as_str()
            .unwrap_or_else(|| "N/A");
        let age_rating_guide = anime_json["data"][index]["attributes"]["ageRatingGuide"]
            .as_str()
            .unwrap_or_else(|| "N/A");

        let status = match status {
            "finished" => "Finished",
            "current" => "Current",
            "tba" => "To Be Announced",
            "unreleased" => "Unreleased",
            "upcoming" => "Upcoming",
            _ => "Unknown",
        };

        Anime {
            name,
            image_url,
            description,
            rating,
            episodes,
            episode_length,
            start_date,
            end_date,
            status,
            age_rating,
            age_rating_guide,
        }
    }
}
