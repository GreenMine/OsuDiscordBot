//use std::collections::HashMap;

use super::types;

const URL: &str = "https://osu.ppy.sh/api/";

pub struct Osu {
    token: String,
}

//TODO: REWRITE TO V2

impl Osu {
    pub fn new(token: String) -> Self {
        Osu { token }
    }

    pub async fn _get_beatmaps(&self, _limit: u32) {
        //        println!(
        //            "Response: {:?}",
        //            self.request("get_beatmaps", &[("limit", &limit.to_string())])
        //                .await
        //                .expect("Error to send request!")
        //        );
    }

    pub fn get_user(&self, name: &str) -> Result<types::User, reqwest::Error> {
        let users: [types::User; 1] = self.request("get_user", &[("u", name)])?;
        let [user] = users;
        Ok(user)
    }

    fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        data: &[(&str, &str)],
    ) -> Result<T, reqwest::Error> {
        reqwest::blocking::get(&self.generate_request(method, data))?
            .json()
    }

    fn generate_request(&self, method: &str, data: &[(&str, &str)]) -> String {
        let mut buffer = URL.to_string() + method + "?";
        data.iter()
            .chain([("k", &self.token[..])].iter())
            .for_each(|(key, value)| buffer += &format!("{}={}&", key, value));
//        println!("{}", buffer);
        buffer
    }
}
