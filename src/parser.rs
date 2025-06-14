use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Trait {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Action {
    pub name: String,
    pub text: String,
    #[serde(rename = "attack")]
    pub attack_info: Option<String>, // Optional field for attack details
    #[serde(rename = "recharge")]
    pub recharge: Option<String>, // Optional field for recharge details
}

#[derive(Debug, Deserialize)]
pub struct Reaction {
    pub name: String,
    pub text: String,
    #[serde(rename = "attack")]
    pub attack_info: Option<String>, // Optional field for attack details
}

#[derive(Debug, Deserialize)]
pub struct Legendary {
    pub name: String,
    pub text: String,
    #[serde(rename = "recharge")]
    pub recharge: Option<String>, // Optional field for recharge details
}

#[derive(Debug, Deserialize)]
pub struct Monster {
    pub name: String,
    pub size: String,
    #[serde(rename = "type")]
    pub monster_type: String, // "type" is a reserved keyword in Rust
    pub alignment: String,
    pub ac: String,
    pub hp: String,
    pub speed: String,
    pub str: u32,
    pub dex: u32,
    pub con: u32,
    pub int: u32,
    pub wis: u32,
    pub cha: u32,
    pub save: Option<String>,
    pub skill: Option<String>,
    pub resist: Option<String>,
    pub vulnerable: Option<String>,
    pub immune: Option<String>,
    pub condition_immune: Option<String>,
    pub senses: String,
    pub passive: Option<u32>,
    pub languages: String,
    pub cr: u32,
    pub description: String,
    #[serde(rename = "trait")]
    pub traits: Vec<Trait>, // Multiple traits
    pub action: Vec<Action>, // Multiple actions
    pub reaction: Vec<Reaction>, // Multiple reactions
    pub legendary: Vec<Legendary>, // Multiple legendary actions
    pub spells: Option<String>, // Optional field for spells
}

pub fn parse_xml_statblocks() -> Result<Vec<Monster>, Box<dyn std::error::Error>> {
    let xml_content = fs::read_to_string("monsters.xml")?;
    let monsters: Vec<Monster> = quick_xml::de::from_str(&xml_content)?;
    Ok(monsters)
}

// fn parse_json_statblock(data: String) -> Monster {

// }