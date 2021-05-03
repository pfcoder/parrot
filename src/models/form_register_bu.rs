use crate::database::PoolType;
use crate::errors::ApiError;
use crate::form_modal::RegisterDataBu;
use crate::schema::form_register_bus;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "form_register_bus"]
pub struct FormRegisterBu {
    pub username: String,
    pub password: String,
    pub passwordq: String,
    pub passworda: String,
    pub usertype: String,
    pub name: String,
    pub address: String,
    pub postcode: String,
    pub personincharge: String,
    pub tel: String,
    pub cell: String,
    pub province: String,
    pub city: String,
    pub mainbusiness: String,
    pub sellingbrand: String,
    pub recommander: String,
    pub recommander_name: String,
    pub subcompany: String,
    pub subcompany_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<RegisterDataBu> for FormRegisterBu {
    fn from(data: RegisterDataBu) -> Self {
        FormRegisterBu {
            username: data.username,
            password: data.password,
            passwordq: data.passwordq,
            passworda: data.passworda,
            usertype: data.usertype,
            name: data.name,
            address: data.address,
            postcode: data.postcode,
            personincharge: data.personincharge,
            tel: data.tel,
            cell: data.cell,
            province: data.province,
            city: data.city,
            mainbusiness: data.mainbusiness,
            sellingbrand: data.sellingbrand,
            recommander: data.recommander,
            recommander_name: data.recommander_name,
            subcompany: data.subcompany,
            subcompany_name: data.subcompany_name,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

/// Create a new form_trail
pub fn create_form_register_bu(pool: &PoolType, new_form: &FormRegisterBu) -> Result<(), ApiError> {
    use crate::schema::form_register_bus::dsl::form_register_bus;

    println!("data: {:?}", new_form);

    let conn = pool.get()?;
    println!("db conn success");
    diesel::insert_into(form_register_bus)
        .values(new_form)
        .execute(&conn)?;
    Ok(())
}
