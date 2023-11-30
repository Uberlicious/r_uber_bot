use crate::Error;

use regex::Regex;

pub async fn get_num_ids() -> Result<usize, Error> {
    let response = reqwest::get("https://superheroapi.com/ids.html")
        .await?
        .text()
        .await?;

    let mut match_counter = 0;
    let re = Regex::new(r"<td>[0-9]*</td>").unwrap();

    let matches: Vec<_> = re.find_iter(response.as_str()).collect();

    Ok(matches.len())
}
