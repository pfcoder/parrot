use crate::form_modal::{BuyData, Forms, RegisterDataBu, RegisterDataPs, RepairData, TrailData};
use crate::mailer::send_mail;
use crate::store::form_store;
use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use handlebars::Handlebars;

#[post("/trail")]
pub async fn submit_trail(
    hb: web::Data<Handlebars<'_>>,
    form_data: web::Form<TrailData>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let trail_data = form_data.into_inner();
    let trail_form: Forms = Forms::Trail(trail_data.clone());
    form_store(&trail_form);

    // build data kvs json map
    let body = match hb.render("table", &trail_data.get_json_maps()) {
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
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[post("/buy")]
pub async fn submit_buy(
    hb: web::Data<Handlebars<'_>>,
    form_data: web::Form<BuyData>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let buy_data = form_data.into_inner();
    let buy_form: Forms = Forms::Buy(buy_data.clone());
    form_store(&buy_form);

    // build data kvs json map
    let body = match hb.render("table", &buy_data.get_json_maps()) {
        Ok(body) => body,
        Err(err) => {
            format!("{}", err)
        }
    };

    let subject = format!(
        "{}, 申请人: {} 联系方式: {}",
        buy_data.form_title, buy_data.name, buy_data.contact
    );

    // TODO: provide text format
    send_mail(body.clone(), body, subject);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[post("/reg_bu")]
pub async fn submit_reg_bu(
    hb: web::Data<Handlebars<'_>>,
    form_data: web::Form<RegisterDataBu>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let regbu_data = form_data.into_inner();
    let regbu_form: Forms = Forms::RegisterBu(regbu_data.clone());
    form_store(&regbu_form);

    // build data kvs json map
    let body = match hb.render("table", &regbu_data.get_json_maps()) {
        Ok(body) => body,
        Err(err) => {
            format!("{}", err)
        }
    };

    let subject = format!(
        "{}, 申请人: {} 联系方式: {}",
        regbu_data.form_title, regbu_data.name, regbu_data.tel
    );

    // TODO: provide text format
    send_mail(body.clone(), body, subject);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[post("/reg_ps")]
pub async fn submit_reg_ps(
    hb: web::Data<Handlebars<'_>>,
    form_data: web::Form<RegisterDataPs>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let regps_data = form_data.into_inner();
    let regps_form: Forms = Forms::RegisterPs(regps_data.clone());
    form_store(&regps_form);

    // build data kvs json map
    let body = match hb.render("table", &regps_data.get_json_maps()) {
        Ok(body) => body,
        Err(err) => {
            format!("{}", err)
        }
    };

    let subject = format!(
        "{}, 申请人: {} 联系方式: {}",
        regps_data.form_title, regps_data.name, regps_data.cell
    );

    // TODO: provide text format
    send_mail(body.clone(), body, subject);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[post("/repair")]
pub async fn submit_repair(
    hb: web::Data<Handlebars<'_>>,
    form_data: web::Form<RepairData>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let repair_data = form_data.into_inner();
    let repair_form: Forms = Forms::Repair(repair_data.clone());
    form_store(&repair_form);

    // build data kvs json map
    let body = match hb.render("table", &repair_data.get_json_maps()) {
        Ok(body) => body,
        Err(err) => {
            format!("{}", err)
        }
    };

    let subject = format!(
        "{}, 申请人: {} 联系方式: {}",
        repair_data.form_title, repair_data.name, repair_data.contact
    );

    // TODO: provide text format
    send_mail(body.clone(), body, subject);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[get("/{form_type}")]
pub async fn form_get(
    hb: web::Data<Handlebars<'_>>,
    web::Path(info): web::Path<String>,
) -> HttpResponse {
    let tplt = format!("form-{}", info);
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
