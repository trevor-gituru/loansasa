use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error as DeError;

fn serialize_naive_datetime<S>(
    date: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&s)
}

fn deserialize_naive_datetime<'de, D>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
        .map_err(DeError::custom)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: i32,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub created_at: NaiveDateTime,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl Session {
    pub fn new(session_id: String, user_id: i32, 
            ip_address: Option<String>, user_agent: 
            Option<String>) -> Self {
        let now = Utc::now().naive_utc();
        Session {
            session_id,
            user_id,
            created_at: now,
            ip_address,
            user_agent,
        }
    }
}