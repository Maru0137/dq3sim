use crate::personality::Personality;
use crate::player::{Player, PlayerInit};
use crate::sex::Sex;
use crate::{job::Job, personality};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GrowthInitEntry {
    job: Job,
    sex: Sex,
    personality: Personality,
    pow: Option<u8>,
    spd: Option<u8>,
    vit: Option<u8>,
    int: Option<u8>,
    lck: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SeedConfig {
    pow: Option<usize>,
    spd: Option<usize>,
    vit: Option<usize>,
    int: Option<usize>,
    lck: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GrowthConfigEntry {
    lv: u8,
    job: Option<Job>,
    personality: Option<Personality>,
    //seed: Option<SeedConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GrowthConfig {
    init: GrowthInitEntry,
    configs: Vec<GrowthConfigEntry>,
}

#[derive(Clone, Debug)]
pub struct PlayerGrowther {
    config: GrowthConfig,
    iter_i: usize,
    player: Player,
}

impl PlayerGrowther {
    pub fn from_config(config: &GrowthConfig) -> Self {
        let mut new_config = GrowthConfig {
            init: config.init.clone(),
            configs: vec![],
        };

        let mut lv = 1;
        let mut job = config.init.job;
        let mut personality = config.init.personality;
        let player = {
            let sex = config.init.sex;
            let pow = 1;
            let spd = 1;
            let vit = config.init.vit.unwrap();
            let int = 1;
            let lck = 1;

            PlayerInit {
                lv: lv,
                pow: pow,
                spd: spd,
                vit: vit,
                int: int,
                lck: lck,
                sex: sex,
                personality: personality,
                job: job,
                ..Default::default()
            }
            .init()
        };

        for entry in &config.configs {
            let new_lv = entry.lv;
            if let Some(new_job) = entry.job {
                lv = 1;
                job = new_job;
            }
            if let Some(new_personality) = entry.personality {
                personality = new_personality;
            }

            for l in lv + 1..=new_lv {
                new_config.configs.push(GrowthConfigEntry {
                    lv: l,
                    job: Some(job),
                    personality: Some(personality),
                })
            }
            lv = new_lv;
        }

        PlayerGrowther {
            config: new_config,
            iter_i: 0,
            player: player,
        }
    }

    pub fn finalize(&mut self) -> Player {
        self.last().unwrap()
    }
}

impl std::iter::Iterator for PlayerGrowther {
    type Item = Player;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_i > self.config.configs.len() {
            return None;
        }

        let ret = Some(self.player.clone());
        self.iter_i += 1;

        if self.iter_i <= self.config.configs.len() {
            let config = &self.config.configs[self.iter_i - 1];
            self.player.personality_change(config.personality.unwrap());

            if self.player.job() != config.job.unwrap() {
                self.player.job_change(config.job.unwrap());
            } else {
                self.player.levelup();
            }
        }

        ret
    }
}

pub fn player_by_config(config_str: &str) -> Player {
    let config: GrowthConfig = serde_json::from_str(config_str).unwrap();

    let lv = 1;
    let job = config.init.job;
    let sex = config.init.sex;
    let personality = config.init.personality;
    let pow = 1;
    let spd = 1;
    let vit = config.init.vit.unwrap();
    let int = 1;
    let lck = 1;

    let mut player = PlayerInit {
        lv: lv,
        pow: pow,
        spd: spd,
        vit: vit,
        int: int,
        lck: lck,
        sex: sex,
        personality: personality,
        job: job,
        ..Default::default()
    }
    .init();

    for entry in &config.configs {
        let target_lv = entry.lv;
        for _ in player.level()..target_lv {
            player.levelup();
        }

        if let Some(job) = entry.job {
            player.job_change(job);
        }

        if let Some(personality) = entry.personality {
            player.personality_change(personality);
        }
    }

    player
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_soldier() {
        let config = r#"
            {
                "init": {
                    "job": "Soldier",
                    "sex": "Man",
                    "personality": "Tough",
                    "vit": 19
                },
                "configs": [
                    {
                        "lv": 22
                    }
                ]
            }
        "#;

        let config: GrowthConfig = serde_json::from_str(config).unwrap();

        let player = PlayerGrowther::from_config(&config);
        println!("{:?}", player);
        let player = PlayerGrowther::from_config(&config);
        println!("{:?}", player);
    }

    #[test]
    fn test_magic_soldier() {
        let config = r#"
            {
                "init": {
                    "job": "Wizard",
                    "sex": "Women",
                    "personality": "Tough",
                    "vit": 15
                },
                "configs": [
                    {
                        "lv": 10
                    },
                    {
                        "lv": 19,
                        "personality": "Solitary"
                    },
                    {
                        "lv": 21,
                        "personality": "Tough"
                    },
                    {
                        "lv": 7,
                        "job": "Soldier",
                        "personality": "Solitary"
                    },
                    {
                        "lv": 23,
                        "personality": "Tough" 
                    }
                ]
            }
        "#;

        let config: GrowthConfig = serde_json::from_str(config).unwrap();

        let mut player = PlayerGrowther::from_config(&config);
        for p in player {
            println!("{:?}", p);
        }
        let player = PlayerGrowther::from_config(&config).finalize();
        println!("{:?}", player);
    }
}
