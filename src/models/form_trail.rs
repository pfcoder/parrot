use crate::database::PoolType;
use crate::errors::ApiError;
use crate::form_modal::TrailData;
use crate::schema::form_trails;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "form_trails"]
pub struct FormTrail {
    pub hospital_name: String,
    pub applicant: String,
    pub city: String,
    pub user: String,
    pub contact: String,
    pub prod: String,
    pub model: String,
    pub month: String,
    pub day: String,
    pub bmonth: String,
    pub bday: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<TrailData> for FormTrail {
    fn from(form_trail: TrailData) -> Self {
        FormTrail {
            hospital_name: form_trail.hospital_name,
            applicant: form_trail.applicant,
            city: form_trail.city,
            user: form_trail.user,
            contact: form_trail.contact,
            prod: form_trail.prod,
            model: form_trail.model,
            month: form_trail.month,
            day: form_trail.day,
            bmonth: form_trail.bmonth,
            bday: form_trail.bday,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

/// Create a new form_trail
pub fn create_form_trail(pool: &PoolType, new_form_trail: &FormTrail) -> Result<(), ApiError> {
    use crate::schema::form_trails::dsl::form_trails;

    println!("data: {:?}", new_form_trail);

    let conn = pool.get()?;
    println!("db conn success");
    diesel::insert_into(form_trails)
        .values(new_form_trail)
        .execute(&conn)?;
    Ok(())
}
