use crate::Error;

use super::Superhero;
use rand::{rngs::StdRng, Rng, SeedableRng};
use regex::Regex;

pub struct SuperheroApi {
    api_key: String,
}

impl SuperheroApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn get_num_ids(&self) -> Result<usize, Error> {
        let response = reqwest::get("https://superheroapi.com/ids.html")
            .await?
            .text()
            .await?;

        let re = Regex::new(r"<td>[0-9]*</td>").unwrap();

        let matches: Vec<_> = re.find_iter(response.as_str()).collect();

        Ok(matches.len())
    }

    pub async fn get_random_superhero(&self) -> Result<Superhero, Error> {
        let mut rng = StdRng::from_entropy();

        let total = self.get_num_ids().await?;

        let num = rng.gen_range(0..=total);

        let url =
            format!("https://superheroapi.com/api/{}/", self.api_key) + num.to_string().as_str();

        let superhero = reqwest::get(url).await?.json::<Superhero>().await?;

        Ok(superhero)
    }
}
