use super::types::{
    self,
    Error
};


const URL: &str = "https://osu.ppy.sh/api/";

pub struct Osu {
    token: String,
}

//TODO: REWRITE TO V2

//Public functions(for users)
impl Osu {
    pub fn new(token: impl AsRef<str>) -> Self {
        Osu {
            token: token.as_ref().to_string(),
        }
    }

    pub async fn _get_beatmaps(&self, _limit: u32) {
        unimplemented!()
    }

    pub fn get_user(&self, name: &str) -> Result<types::User, Error> {
        let users: [types::User; 1] = self.request("get_user", &[("u", name)])?;
        let [user] = users;
        Ok(user)
    }
}

//Private functions
impl Osu {
    //Response time ~600ms:thinking:
    fn request<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
        method: &str,
        data: &[(&str, &str)],
    ) -> Result<T, Error> {
        let json = reqwest::blocking::get(&self.generate_request(method, data))?.json();
        if let Err(_) = json {
            return Err(Error::Osu(types::error::OsuError {error: "Unable to get user!".to_string()}));
        }
        Ok(json.unwrap())
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
