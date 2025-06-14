use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;

#[allow(dead_code)]
pub struct Speeds {
    pub walking: i32,
    pub climbing: Option<i32>,
    pub flying: Option<i32>
}

#[allow(dead_code)]
pub struct AbilityScore {
    pub name: String,
    pub score: Option<i32>,
    pub modifier: i32,
}

#[allow(dead_code)]
pub struct Skill {
    pub name: String,
    pub modifier: i32,
}

#[allow(dead_code)]
pub struct Senses {
    pub passive: i32,
    pub darkvision: Option<i32>,
    pub blindsight: Option<i32>,
    pub truesight: Option<i32>,
    pub tremorsense: Option<i32>
}

#[allow(dead_code)]
pub struct SpellCaster {
    pub save_dc: i32,
    pub hit_modifier: i32,
    pub cantrips: Vec<String>,
    pub first_level: Option<Vec<String>>,
    pub second_level: Option<Vec<String>>,
    pub third_level: Option<Vec<String>>,
    pub fourth_level: Option<Vec<String>>,
    pub fifth_level: Option<Vec<String>>,
    pub sixth_level: Option<Vec<String>>,
    pub seventh_level: Option<Vec<String>>,
    pub eighth_level: Option<Vec<String>>,
    pub nineth_level: Option<Vec<String>>,
    
}

#[allow(dead_code)]
pub struct Monster {
    pub name: String,
    pub hp: String,
    pub ac: i32,
    pub cr: i32,
    pub speed: Speeds,
    pub ability_scores: Vec<AbilityScore>,
    pub saving_throws: Option<Vec<AbilityScore>>,
    pub skills: Option<Vec<Skill>>,
    pub resistances: Option<Vec<String>>,
    pub immunities: Option<Vec<String>>,
    pub condition_immunities: Option<Vec<String>>,
    pub senses: Senses,
    pub traits: Option<Vec<String>>,
    pub spellcasting: Option<Vec<String>>,
    pub actions: Vec<String>,
    pub legendary_actions: Option<Vec<String>>,
    pub lair_actions: Option<Vec<String>>,
    pub mythic_actions: Option<Vec<String>>
}

// fn parse_xml_statblock(name: String) -> Monster {
//     let file = fs::read_to_string("Complete_Compendium.xml").expect("Unable to open!");
//     let mut reader = Reader::from_str(file.as_str());
//     reader.config_mut().trim_text(true);

//     let mut monster: Monster;
//     monster.name = name;
//     monster.ac = 16;

//     monster
// }

// fn parse_json_statblock(data: String) -> Monster {

// }