use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
// use std::fmt::format;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(get_index))
        .route("/gcd", web::post().to(post_gcd))
    });

    println!("serving on http://localhost:3000...");
    let result = server.bind("127.0.0.1:3000").expect("error binding server to address").run().await;
    println!("RESULT is : {}", result.is_err());
}


async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse{
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("Computing the GCD with zero is boring.";);
    }
    let response = format!("The greatest common devisor of the numbers {} and {} is : \n <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m)).as_str();
    HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64{
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}