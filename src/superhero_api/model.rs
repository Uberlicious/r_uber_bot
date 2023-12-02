use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Superhero {
    pub response: String,
    pub id: String,
    pub name: String,
    pub powerstats: Powerstats,
    pub biography: Biography,
    pub appearance: Appearance,
    pub work: Work,
    pub connections: Connections,
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Powerstats {
    pub intelligence: String,
    pub strength: String,
    pub speed: String,
    pub durability: String,
    pub power: String,
    pub combat: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Biography {
    #[serde(rename = "full-name")]
    pub full_name: String,
    #[serde(rename = "alter-egos")]
    pub alter_egos: String,
    pub aliases: Vec<String>,
    #[serde(rename = "place-of-birth")]
    pub place_of_birth: String,
    #[serde(rename = "first-appearance")]
    pub first_appearance: String,
    pub publisher: String,
    pub alignment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Appearance {
    pub gender: String,
    pub race: String,
    pub height: Vec<String>,
    pub weight: Vec<String>,
    #[serde(rename = "eye-color")]
    pub eye_color: String,
    #[serde(rename = "hair-color")]
    pub hair_color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    pub occupation: String,
    pub base: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connections {
    #[serde(rename = "group-affiliation")]
    pub group_affiliation: String,
    pub relatives: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
}
