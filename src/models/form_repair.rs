use crate::database::PoolType;
use crate::errors::ApiError;
use crate::form_modal::RepairData;
use crate::schema::form_repairs;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "form_repairs"]
pub struct FormRepair {
    pub hospital_name: String,
    pub city: String,
    pub name: String,
    pub contact: String,
    pub prod: String,
    pub model: String,
    pub serial: String,
    pub year: String,
    pub month: String,
    pub question: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<RepairData> for FormRepair {
    fn from(data: RepairData) -> Self {
        FormRepair {
            hospital_name: data.hospital_name,
            city: data.city,
            name: data.name,
            contact: data.contact,
            prod: data.prod,
            model: data.model,
            serial: data.serial,
            year: data.year,
            month: data.month,
            question: data.question,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

/// Create a new form_trail
pub fn create_form_repair(pool: &PoolType, new_form: &FormRepair) -> Result<(), ApiError> {
    use crate::schema::form_repairs::dsl::form_repairs;

    println!("data: {:?}", new_form);

    let conn = pool.get()?;
    println!("db conn success");
    diesel::insert_into(form_repairs)
        .values(new_form)
        .execute(&conn)?;
    Ok(())
}
