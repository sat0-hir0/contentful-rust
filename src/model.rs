use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Entry<T> {
    sys: SystemProperties,
    fields: T,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct SystemProperties {
    id: String,
    version: Option<i32>,
    revision: Option<i32>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
