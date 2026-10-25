use actix_web::{web, get, post,  App, Responder, HttpServer, HttpResponse};
use serde::{Deserialize};

use std::env;
use std::io::Result;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m!= 0);
    while m != 0 {
        if  m< n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("serving on http://localhost:3000...");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(greet)
            .service(gcd_service)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    let contents = r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
      <input type="text" name="n"/>
      <input type="text" name="m"/>
      <button type="submit">Compute GCD</button>
    </form>
    "#;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(contents)
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>)  -> impl Responder {
    format!("Hello, {}", name)
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[post("/gcd")]
async fn gcd_service(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }
    let response = format!("The greatest common divisor of the numbers \
                            {} and {} is <b>{}</b>\n",
                           form.n, form.m, gcd(form.n, form.m));
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

#[warn(unused)]
fn main_cli() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(
            2 * 3 * 5 * 11 * 17,
            3 * 7 * 11 * 13 * 19
        ),
            3 * 11);
   } 
}
