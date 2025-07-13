use std::net::IpAddr;

use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::{
    crypto::hash::{HashedData, Salt},
    database::SerialId,
};

#[derive(FromRow)]
pub struct Admin {
    id: SerialId,
    username: String,
    hashed_pass: HashedData,
    salt: Salt,
    created_at: DateTime<Utc>,
    issued_by: Option<IpAddr>,
}

#[derive(FromRow)]
pub struct InsertAdmin {
    pub username: String,
    pub hashed_pass: HashedData,
    pub salt: Salt,
    pub issued_by: Option<IpAddr>,
}
