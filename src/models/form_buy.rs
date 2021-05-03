use crate::database::PoolType;
use crate::errors::ApiError;
use crate::form_modal::BuyData;
use crate::schema::form_buys;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "form_buys"]
pub struct FormBuy {
    pub per_com: String,
    pub company: String,
    pub city: String,
    pub name: String,
    pub contact: String,
    pub prod: String,
    pub model: String,
    pub amount: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<BuyData> for FormBuy {
    fn from(data: BuyData) -> Self {
        FormBuy {
            per_com: data.per_com,
            company: data.company,
            city: data.city,
            name: data.name,
            contact: data.contact,
            prod: data.prod,
            model: data.model,
            amount: data.amount,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

/// Create a new form_trail
pub fn create_form_buy(pool: &PoolType, new_form: &FormBuy) -> Result<(), ApiError> {
    use crate::schema::form_buys::dsl::form_buys;

    println!("data: {:?}", new_form);

    let conn = pool.get()?;
    println!("db conn success");
    diesel::insert_into(form_buys)
        .values(new_form)
        .execute(&conn)?;
    Ok(())
}
