
use super::string_parse::*;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(deserialize_with="parse_string_as_u64")]pub user_id: u64,
    pub username: String,
    pub join_date: String,
    #[serde(deserialize_with="parse_string_as_u64")]pub count300: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub count50: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub playcount: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub ranked_score: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub total_score: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub pp_rank: u64,
    #[serde(deserialize_with="parse_string_as_f32")]pub level: f32,
    #[serde(deserialize_with="parse_string_as_f32")]pub pp_raw: f32,
    #[serde(deserialize_with="parse_string_as_f32")]pub accuracy: f32,
    #[serde(deserialize_with="parse_string_as_u64")]pub count_rank_ss: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub count_rank_ssh: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub count_rank_s: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub count_rank_sh: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub count_rank_a: u64,
    pub country: String,
    #[serde(deserialize_with="parse_string_as_u64")]pub total_seconds_played: u64,
    #[serde(deserialize_with="parse_string_as_u64")]pub pp_country_rank: u64,
    pub events: Vec<Event>
}


#[derive(Deserialize, Debug)]
pub struct Event {
    display_html: String,
    #[serde(deserialize_with="parse_string_as_u64")]beatmap_id: u64,
    #[serde(deserialize_with="parse_string_as_u64")]beatmapset_id: u64,
    date: String,
    epicfactor: String
}


impl User {
    pub fn get_avatar(&self) -> String {
        format!("https://a.ppy.sh/{}", self.user_id)
    }

    pub fn get_profile(&self) -> String {
        format!("https://osu.ppy.sh/users/{}", self.user_id)
    }
}

//{\"user_id\":\"14114565\",\"username\":\"XLADoMAZ\",\"join_date\":\"2019-03-17 13:42:48\",\"count300\":\"52277\",\"count100\":\"16287\",\"count50\":\"3330\",\"playcount\":\"654\",\"ranked_score\":\"34051242\",\"total_score\":\"80904258\",\"pp_rank\":\"1108911\",\"level\":\"23.2289\",\"pp_raw\":\"225.878\",\"accuracy\":\"83.60210418701172\",\"count_rank_ss\":\"0\",\"count_rank_ssh\":\"0\",\"count_rank_s\":\"0\",\"count_rank_sh\":\"0\",\"count_rank_a\":\"7\",\"country\":\"RU\",\"total_seconds_played\":\"38541\",\"pp_country_rank\":\"125109\",\"events\":[]}]
