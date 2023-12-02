use crate::Error;

use super::Superhero;
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
    let superhero = reqwest::get("https://superheroapi.com/api/10158934816710166/731")
        .await?
        .json::<Superhero>()
        .await?;

    println!("{:?}", superhero);

    Ok(superhero)
}
