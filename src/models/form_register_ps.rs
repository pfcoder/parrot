use crate::database::PoolType;
use crate::errors::ApiError;
use crate::form_modal::RegisterDataPs;
use crate::schema::form_register_pss;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "form_register_pss"]
pub struct FormRegisterPs {
    pub username: String,
    pub password: String,
    pub passwordq: String,
    pub passworda: String,
    pub usertype: String,
    pub name: String,
    pub year: String,
    pub month: String,
    pub company: String,
    pub title: String,
    pub expertise: String,
    pub cell: String,
    pub identity: String,
    pub recommander: String,
    pub recommander_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<RegisterDataPs> for FormRegisterPs {
    fn from(data: RegisterDataPs) -> Self {
        FormRegisterPs {
            username: data.username,
            password: data.password,
            passwordq: data.passwordq,
            passworda: data.passworda,
            usertype: data.usertype,
            name: data.name,
            year: data.year,
            month: data.month,
            company: data.company,
            title: data.title,
            expertise: data.expertise,
            cell: data.cell,
            identity: data.id,
            recommander: data.recommander,
            recommander_name: data.recommander_name,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

/// Create a new form_trail
pub fn create_form_register_ps(pool: &PoolType, new_form: &FormRegisterPs) -> Result<(), ApiError> {
    use crate::schema::form_register_pss::dsl::form_register_pss;

    println!("data: {:?}", new_form);

    let conn = pool.get()?;
    println!("db conn success");
    diesel::insert_into(form_register_pss)
        .values(new_form)
        .execute(&conn)?;
    Ok(())
}
