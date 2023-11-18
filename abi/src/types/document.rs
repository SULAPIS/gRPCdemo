use chrono::{DateTime, Utc};
use prost_wkt::MessageSerde;
use prost_wkt_types::{Struct, Timestamp};
use serde_json::Value;
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Row};

use crate::{Document, Error, Validator};

use super::validate_range;

impl Validator for Document {
    fn validate(&self) -> Result<(), Error> {
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId(self.user_id.clone()));
        }

        validate_range(self.created_at.as_ref(), self.created_at.as_ref())?;
        Ok(())
    }
}

impl FromRow<'_, PgRow> for Document {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let id: Uuid = row.get("id");
        let user_id: Uuid = row.get("user_id");
        let data: Value = row.get("data");
        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: DateTime<Utc> = row.get("updated_at");

        let data: Struct = serde_json::from_value(data).unwrap();

        Ok(Self {
            id: id.to_string(),
            user_id: user_id.to_string(),
            data: Some(data),
            created_at: Some(Timestamp::from(created_at)),
            updated_at: Some(Timestamp::from(updated_at)),
        })
    }
}
