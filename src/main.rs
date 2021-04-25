#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use crate::form_modal::{BuyData, Forms, RegisterDataBu, RegisterDataPs, TrailData};
use crate::mailer::send_mail;
use crate::store::form_store;
use actix_files::Files;
use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::{header, Method, StatusCode};
use actix_web::middleware;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use handlebars::Handlebars;

mod form_modal;
mod mailer;
mod store;

#[post("/submit/trail")]
async fn submit_trail(
    hb: web::Data<Handlebars<'_>>,
    form_data: web::Form<TrailData>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let trail_data = form_data.into_inner();
    let trail_form: Forms = Forms::Trail(trail_data.clone());
    form_store(&trail_form);

    // send mail
    //let tplt = format!("mail-{}", trail_form.get_type());
    let tplt = "table";
    println!("tplt: {}", tplt);

    // build data kvs json map

    let body = match hb.render(&tplt, &trail_data) {
        Ok(body) => body,
        Err(err) => {
            format!("{}", err)
        }
    };

    let subject = format!(
        "{}, 申请人: {} 联系方式: {}",
        trail_data.form_title, trail_data.applicant, trail_data.contact
    );

    // TODO: provide text format
    send_mail(body.clone(), body, subject);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/templates/submit-ok.html"))
}

#[get("/form/{form_type}")]
async fn form(hb: web::Data<Handlebars<'_>>, web::Path(info): web::Path<(String)>) -> HttpResponse {
    let form_type = info;
    let tplt = format!("form-{}", form_type);
    println!("tplt: {}", tplt);

    // prepare form data
    let data = json!({
        "token": "abc", // TODO: test only
    });

    let body = match hb.render(&tplt, &data) {
        Ok(body) => body,
        Err(err) => {
            format!("{}", err)
        }
    };
    //println!("body: {}", body);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(body)
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str()
            });
            let body = hb.render("error", &data);

            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("Parrot server start on 8080");

    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            .wrap(middleware::Logger::default())
            .app_data(handlebars_ref.clone())
            .service(form)
            .service(submit_trail)
            .service(Files::new("/asset", "static/asset/").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
