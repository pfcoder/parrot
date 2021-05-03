use crate::cache::{delete, get, set, Cache};
use crate::database::PoolType;
use crate::form_modal::{BuyData, Forms, RegisterDataBu, RegisterDataPs, RepairData, TrailData};
use crate::mailer::send_mail;
use crate::models::form_buy::create_form_buy;
use crate::models::form_register_bu::create_form_register_bu;
use crate::models::form_register_ps::create_form_register_ps;
use crate::models::form_repair::create_form_repair;
use crate::models::form_trail::create_form_trail;
use crate::store::form_store;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, web::block, HttpResponse, Responder};
use handlebars::Handlebars;
use uuid::Uuid;

#[post("/trail")]
pub async fn submit_trail(
    hb: web::Data<Handlebars<'_>>,
    cache: Cache,
    pool: web::Data<PoolType>,
    form_data: web::Form<TrailData>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let trail_data = form_data.into_inner();

    match get(cache.clone(), &trail_data.token).await {
        Ok(_v) => {
            // token verify pass
            delete(cache, &trail_data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", "/form/trail")
                .finish();
        }
    }

    let trail_form: Forms = Forms::Trail(trail_data.clone());
    form_store(&trail_form);

    // write databse
    let clone_date = trail_data.clone();
    match block(move || create_form_trail(&pool, &clone_date.into())).await {
        Ok(()) => println!("write db success"),
        Err(e) => println!("write db error: {}", e),
    }

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
    block(move || send_mail(body.clone(), body, subject)).await;

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[post("/buy")]
pub async fn submit_buy(
    hb: web::Data<Handlebars<'_>>,
    cache: Cache,
    pool: web::Data<PoolType>,
    form_data: web::Form<BuyData>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let buy_data = form_data.into_inner();

    match get(cache.clone(), &buy_data.token).await {
        Ok(_v) => {
            // token verify pass
            delete(cache, &buy_data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", "/form/buy")
                .finish();
        }
    }

    let buy_form: Forms = Forms::Buy(buy_data.clone());
    form_store(&buy_form);

    // write databse
    let clone_date = buy_data.clone();
    match block(move || create_form_buy(&pool, &clone_date.into())).await {
        Ok(()) => println!("write db success"),
        Err(e) => println!("write db error: {}", e),
    }

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
    block(move || send_mail(body.clone(), body, subject)).await;

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[post("/reg_bu")]
pub async fn submit_reg_bu(
    hb: web::Data<Handlebars<'_>>,
    cache: Cache,
    pool: web::Data<PoolType>,
    form_data: web::Form<RegisterDataBu>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let regbu_data = form_data.into_inner();

    match get(cache.clone(), &regbu_data.token).await {
        Ok(_v) => {
            // token verify pass
            delete(cache, &regbu_data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", "/form/reg_bu")
                .finish();
        }
    }

    let regbu_form: Forms = Forms::RegisterBu(regbu_data.clone());
    form_store(&regbu_form);

    // write databse
    let clone_date = regbu_data.clone();
    match block(move || create_form_register_bu(&pool, &clone_date.into())).await {
        Ok(()) => println!("write db success"),
        Err(e) => println!("write db error: {}", e),
    }

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
    block(move || send_mail(body.clone(), body, subject)).await;

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[post("/reg_ps")]
pub async fn submit_reg_ps(
    hb: web::Data<Handlebars<'_>>,
    cache: Cache,
    pool: web::Data<PoolType>,
    form_data: web::Form<RegisterDataPs>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let regps_data = form_data.into_inner();

    match get(cache.clone(), &regps_data.token).await {
        Ok(_v) => {
            // token verify pass
            delete(cache, &regps_data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", "/form/reg_ps")
                .finish();
        }
    }

    let regps_form: Forms = Forms::RegisterPs(regps_data.clone());
    form_store(&regps_form);

    // write databse
    let clone_date = regps_data.clone();
    match block(move || create_form_register_ps(&pool, &clone_date.into())).await {
        Ok(()) => println!("write db success"),
        Err(e) => println!("write db error: {}", e),
    }

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
    block(move || send_mail(body.clone(), body, subject)).await;

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[post("/repair")]
pub async fn submit_repair(
    hb: web::Data<Handlebars<'_>>,
    cache: Cache,
    pool: web::Data<PoolType>,
    form_data: web::Form<RepairData>,
) -> impl Responder {
    println!("application: {:?}", form_data);

    // write form_data into csv file
    let repair_data = form_data.into_inner();

    match get(cache.clone(), &repair_data.token).await {
        Ok(_v) => {
            // token verify pass
            delete(cache, &repair_data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", "/form/repair")
                .finish();
        }
    }

    let repair_form: Forms = Forms::Repair(repair_data.clone());
    form_store(&repair_form);

    // write databse
    let clone_date = repair_data.clone();
    match block(move || create_form_repair(&pool, &clone_date.into())).await {
        Ok(()) => println!("write db success"),
        Err(e) => println!("write db error: {}", e),
    }

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
    block(move || send_mail(body.clone(), body, subject)).await;

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/templates/submit-ok.html"))
}

#[get("/{form_type}")]
pub async fn form_get(
    hb: web::Data<Handlebars<'_>>,
    cache: Cache,
    web::Path(info): web::Path<String>,
) -> HttpResponse {
    let tplt = format!("form-{}", info);
    println!("tplt: {}", tplt);

    let token = Uuid::new_v4().to_string();
    println!("token: {}", token);

    set(cache, &token, "1").await.unwrap();

    // prepare form data
    let data = json!({
        "token": token,
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
