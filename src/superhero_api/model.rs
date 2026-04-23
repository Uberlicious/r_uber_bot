use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::deserialize_number_from_string_or_default;

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
    #[serde(
        default = "default_num",
        deserialize_with = "deserialize_number_from_string_or_default"
    )]
    pub intelligence: u32,

    #[serde(
        default = "default_num",
        deserialize_with = "deserialize_number_from_string_or_default"
    )]
    pub strength: u32,

    #[serde(
        default = "default_num",
        deserialize_with = "deserialize_number_from_string_or_default"
    )]
    pub speed: u32,

    #[serde(
        default = "default_num",
        deserialize_with = "deserialize_number_from_string_or_default"
    )]
    pub durability: u32,

    #[serde(
        default = "default_num",
        deserialize_with = "deserialize_number_from_string_or_default"
    )]
    pub power: u32,

    #[serde(
        default = "default_num",
        deserialize_with = "deserialize_number_from_string_or_default"
    )]
    pub combat: u32,
}

fn default_num() -> u32 {
    0
}

impl Powerstats {
    pub fn as_array(&self) -> [u32; 6] {
        [
            self.intelligence,
            self.strength,
            self.speed,
            self.durability,
            self.power,
            self.combat,
        ]
    }

    pub fn overall(&self) -> u32 {
        self.as_array().iter().sum()
    }
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
