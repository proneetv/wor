use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParams {
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

    println!("serving on http://localhost:8080...");

    server
        .bind("127.0.0.1:8080").expect("error binding server to addr")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
<title>gcd calculator</title>
<form action="/gcd" method="post">
<input type="text" name="n"/>
<input type="text" name="m"/>
<button type="submit">compute gcd</button>
</form>
              "#,
            )

}

async fn post_gcd(form: web::Form<GcdParams>) -> HttpResponse {

    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Zeroes can't make it to gcd");
    }

    let response = format!("Gcd of numbers {} and {} is <b>{}</b>\n",
                           form.n, form.m, gcd(form.n, form.m));
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
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

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2*3*4*5, 2*3*5), 2*3*5);
}
