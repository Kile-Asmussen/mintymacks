use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::notation::uci::{
    engine::{EngineOption, OptionType, SpinType},
    gui::OptVal,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Profile {
    #[serde(untagged)]
    Player(PlayerProfile),
    #[serde(untagged)]
    Engine(EngineProfile),
}

impl Profile {
    pub fn name(&self) -> &str {
        match self {
            Self::Player(player_profile) => &player_profile.human.name,
            Self::Engine(engine_profile) => &engine_profile.engine.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub human: PlayerMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerMetadata {
    pub name: String,
    pub title: String,
    pub elo: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineProfile {
    pub engine: EngineMetadata,
    pub options: IndexMap<String, OptVal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineMetadata {
    pub name: String,
    pub author: String,
    pub command: (PathBuf, Vec<String>),
    #[serde(default)]
    pub log: bool,
}

impl EngineMetadata {
    pub fn engine_profile_toml(&self, options: &IndexMap<String, EngineOption>) -> String {
        let mut res = String::new();

        res += "[engine]\n";
        res += &toml::to_string(self).expect("Unable to render TOML");

        res += "\n[options]\n";

        for (k, v) in options {
            match &v.option_type {
                OptionType::Check(ot) => {
                    if let Some(val) = ot.value {
                        res += &format!("{k} = {val} # true or false, default {}\n", ot.default);
                    } else {
                        res += &format!("# {k} = {0} # true or false, default {0}\n", ot.default);
                    }
                }
                OptionType::Spin(ot) => {
                    let SpinType {
                        min,
                        max,
                        default,
                        value,
                    } = *ot;

                    if let Some(val) = value {
                        res +=
                            &format!("{k} = {val} # between {min} and {max}, default {default}\n",);
                    } else {
                        res += &format!(
                            "# {k} = {default} # between {min} and {max}, default {default}\n"
                        );
                    }
                }
                OptionType::Combo(ot) => {
                    let default = &ot.default;
                    let variants = ot
                        .variants
                        .iter()
                        .map(|s| format!("\"{s}\""))
                        .collect::<Vec<_>>()
                        .join(", ");

                    if let Some(val) = &ot.value {
                        res += &format!(
                            "{k} = \"{val}\" # default \"{default}\", can be one of {variants}\n",
                        );
                    } else {
                        res += &format!(
                            "# {k} = \"{default}\" # default \"{default}\", can be one of {variants}\n",
                        );
                    }
                }
                OptionType::String(ot) => {
                    if let Some(val) = &ot.value {
                        res += &format!("{k} = \"{val}\"\n# ^^^^ default \"{}\"\n", ot.default);
                    } else {
                        res += &format!("# {k} = \"{0}\"\n", ot.default);
                    }
                }
                OptionType::Button(_) => continue,
            };
            res += "\n";
        }

        res
    }
}
