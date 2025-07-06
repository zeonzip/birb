#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct Salt(Vec<u8>);

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct HashedData(Vec<u8>);

impl Salt {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl HashedData {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}
