#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate redis_async;

mod config;
mod form_modal;
pub mod handlers;
mod mailer;
mod routes;
mod server;
mod store;

use crate::server::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
