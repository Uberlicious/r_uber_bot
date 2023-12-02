use crate::Error;

use super::Superhero;
use rand::{rngs::StdRng, Rng, SeedableRng};
use regex::Regex;

pub async fn get_num_ids() -> Result<usize, Error> {
    let response = reqwest::get("https://superheroapi.com/ids.html")
        .await?
        .text()
        .await?;

    let re = Regex::new(r"<td>[0-9]*</td>").unwrap();

    let matches: Vec<_> = re.find_iter(response.as_str()).collect();

    Ok(matches.len())
}

pub async fn get_random_superhero() -> Result<Superhero, Error> {
    let mut rng = StdRng::from_entropy();

    let total = get_num_ids().await?;

    let num = rng.gen_range(0..=total);

    let url =
        "https://superheroapi.com/api/10158934816710166/".to_owned() + num.to_string().as_str();

    let superhero = reqwest::get(url).await?.json::<Superhero>().await?;

    Ok(superhero)
}
