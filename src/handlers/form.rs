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
use actix::Arbiter;
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

    let data = form_data.into_inner();
    let redirect = "/form/trail";

    match get(cache.clone(), &data.token).await {
        Ok(_v) => {
            // token verify pass
            let _ = delete(cache, &data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", redirect)
                .finish();
        }
    }

    let body = hb
        .render("table", &data.get_json_maps())
        .unwrap_or("处理中...".to_string());

    Arbiter::spawn(async {
        data_process(body, pool, Forms::Trail(data)).await;
    });

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

    let data = form_data.into_inner();
    let redirect = "/form/buy";

    match get(cache.clone(), &data.token).await {
        Ok(_v) => {
            // token verify pass
            let _ = delete(cache, &data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", redirect)
                .finish();
        }
    }

    let body = hb
        .render("table", &data.get_json_maps())
        .unwrap_or("处理中...".to_string());

    Arbiter::spawn(async {
        data_process(body, pool, Forms::Buy(data)).await;
    });

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

    let data = form_data.into_inner();
    let redirect = "/form/reg_bu";

    match get(cache.clone(), &data.token).await {
        Ok(_v) => {
            // token verify pass
            let _ = delete(cache, &data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", redirect)
                .finish();
        }
    }

    let body = hb
        .render("table", &data.get_json_maps())
        .unwrap_or("处理中...".to_string());

    Arbiter::spawn(async {
        data_process(body, pool, Forms::RegisterBu(data)).await;
    });

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
    let data = form_data.into_inner();
    let redirect = "/form/reg_ps";

    match get(cache.clone(), &data.token).await {
        Ok(_v) => {
            // token verify pass
            let _ = delete(cache, &data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", redirect)
                .finish();
        }
    }

    let body = hb
        .render("table", &data.get_json_maps())
        .unwrap_or("处理中...".to_string());

    Arbiter::spawn(async {
        data_process(body, pool, Forms::RegisterPs(data)).await;
    });

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

    let data = form_data.into_inner();
    let redirect = "/form/repair";

    match get(cache.clone(), &data.token).await {
        Ok(_v) => {
            // token verify pass
            let _ = delete(cache, &data.token).await;
        }
        Err(_e) => {
            // token incorrect, redirect page
            return HttpResponse::PermanentRedirect()
                .header("Location", redirect)
                .finish();
        }
    }

    let body = hb
        .render("table", &data.get_json_maps())
        .unwrap_or("处理中...".to_string());

    Arbiter::spawn(async {
        data_process(body, pool, Forms::Repair(data)).await;
    });

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

async fn data_process(mail_body: String, pool: web::Data<PoolType>, forms: Forms) {
    match form_store(&forms) {
        Ok(_v) => println!("write csv successfully"),
        Err(e) => println!("write csv fail:{}", e),
    }
    // extract data
    let subject;

    match forms {
        Forms::Trail(data) => {
            subject = format!(
                "{}, 申请人: {} 联系方式: {}",
                data.form_title, data.applicant, data.contact
            );

            match block(move || create_form_trail(&pool, &data.clone().into())).await {
                Ok(()) => println!("write db success"),
                Err(e) => println!("write db error: {}", e),
            }
        }
        Forms::Buy(data) => {
            subject = format!(
                "{}, 申请人: {} 联系方式: {}",
                data.form_title, data.name, data.contact
            );

            match block(move || create_form_buy(&pool, &data.clone().into())).await {
                Ok(()) => println!("write db success"),
                Err(e) => println!("write db error: {}", e),
            }
        }
        Forms::Repair(data) => {
            subject = format!(
                "{}, 申请人: {} 联系方式: {}",
                data.form_title, data.name, data.contact
            );

            match block(move || create_form_repair(&pool, &data.clone().into())).await {
                Ok(()) => println!("write db success"),
                Err(e) => println!("write db error: {}", e),
            }
        }
        Forms::RegisterPs(data) => {
            subject = format!(
                "{}, 申请人: {} 联系方式: {}",
                data.form_title, data.name, data.cell
            );

            match block(move || create_form_register_ps(&pool, &data.clone().into())).await {
                Ok(()) => println!("write db success"),
                Err(e) => println!("write db error: {}", e),
            }
        }
        Forms::RegisterBu(data) => {
            subject = format!(
                "{}, 申请人: {} 联系方式: {}",
                data.form_title, data.name, data.tel
            );

            match block(move || create_form_register_bu(&pool, &data.clone().into())).await {
                Ok(()) => println!("write db success"),
                Err(e) => println!("write db error: {}", e),
            }
        }
    }

    let _ = block(move || send_mail(mail_body.clone(), mail_body, subject)).await;
}
