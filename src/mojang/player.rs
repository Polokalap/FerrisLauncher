use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use serde::Deserialize;
use serde_json::Value;
use base64::{engine::general_purpose, DecodeError, Engine as _};
use crate::logger::info;

static user_cache: OnceLock<Mutex<HashMap<String, User>>> = OnceLock::new();
static profile_cache: OnceLock<Mutex<HashMap<String, Profile>>> = OnceLock::new();

fn usercache() -> &'static Mutex<HashMap<String, User>> {
    user_cache.get_or_init(|| Mutex::new(HashMap::new()))
}
fn profilecache() -> &'static Mutex<HashMap<String, Profile>> {
    profile_cache.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Clone)]
pub struct User {
    pub(crate) name: String,
    pub(crate) uuid: String,
    pub(crate) online: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Profile {
    pub(crate) name: String,
    pub(crate) id: String,
    pub(crate) properties: Vec<ProfileProperty>,
    #[serde(rename = "profileActions")]
    pub profile_actions: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("failed to parse response: {0}")]
    Json(#[from] serde_json::Error),
    #[error("No such user!")]
    NotFound(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextureProfile {
    pub timestamp: u64,

    #[serde(rename = "profileId")]
    pub profile_id: String,

    #[serde(rename = "profileName")]
    pub profile_name: String,

    pub textures: Textures,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Textures {
    #[serde(rename = "SKIN")]
    pub skin: Skin,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Skin {
    pub url: String,
    #[serde(default)]
    pub metadata: Option<Metadata>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Metadata {
    pub model: String,
}

pub async fn fetch_raw(ign: String) -> Result<User, reqwest::Error> {

    // Cache

    let key = ign.to_lowercase();

    if let Some(cached_user) = usercache().lock().unwrap().get(&key) {
        return Ok(cached_user.clone());
    }

    let response = reqwest::get(&format!("https://api.mojang.com/users/profiles/minecraft/{}", &ign)).await?;
    let status = response.status();
    let body = response.text().await?;
    let json: Value = serde_json::from_str(&body).unwrap();

    let name = if json["name"].is_null() { ign.clone() } else { json["name"].as_str().unwrap().to_string() };
    let uuid = if json["id"].is_null() { String::from("0") } else { json["id"].as_str().unwrap().to_string() };
    let online: bool = status == 200;

    let user: User = User {
        name,
        uuid,
        online,
    };

    usercache().lock().unwrap().insert(key, user.clone());

    Ok(user)

}

pub async fn fetch_profile(name: String) -> Result<Profile, ApiError> {

    // Cache
    let key = name.to_lowercase();

    if let Some(cached_profile) = profilecache().lock().unwrap().get(&key) {
        return Ok(cached_profile.clone());
    }

    let user = fetch_raw(name.clone()).await?;
    let uuid = user.uuid;
    let profile: Profile;

    if user.online {

        let response = reqwest::get(&format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", &uuid)).await?;
        let body = response.text().await?;
        // let json: serde_json::Value = serde_json::from_str(&body)?;

        profile = serde_json::from_str::<Profile>(&body)?;

    } else {

        profile = Profile {
            name,
            id: uuid,
            properties: vec![
                ProfileProperty {
                    name: String::from("texture"),
                    value: String::from("ewogICJ0aW1lc3RhbXAiIDogMTc4NDkxMzE4NDA2NiwKICAicHJvZmlsZUlkIiA6ICJjMDZmODkwNjRjOGE0OTExOWMyOWVhMWRiZDFhYWI4MiIsCiAgInByb2ZpbGVOYW1lIiA6ICJNSEZfU3RldmUiLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZDVjNGVlNWNlMjBhZWQ5ZTMzZTg2NmM2NmNhYTM3MTc4NjA2MjM0YjM3MjEwODRiZjAxZDEzMzIwZmIyZWIzZiIsCiAgICAgICJtZXRhZGF0YSIgOiB7CiAgICAgICAgIm1vZGVsIiA6ICJzbGltIgogICAgICB9CiAgICB9CiAgfQp9")
                }
            ],
            profile_actions: vec![]
        }

    }

    profilecache().lock().unwrap().insert(key, profile.clone());

    Ok(profile)

}

pub async fn get_texture_value(name: String) -> Result<TextureProfile, Box<dyn std::error::Error + Send + Sync>> {

    let profile: Profile = fetch_profile(name.clone()).await?;
    let base64: String = profile.properties[0].value.clone();
    let decoded = general_purpose::STANDARD.decode(base64)?;
    let decoded_str = String::from_utf8(decoded)?;

    let texture_profile: TextureProfile = serde_json::from_str(&decoded_str)?;

    Ok(texture_profile)

}