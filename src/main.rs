pub(crate) mod pon;
use pon::get_pon_full;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use tide::{ Request};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/healthz").get(healthz);
    app.at("/test").get(handle_get);
    app.at("pon").get(get_pon);

    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    app.listen(format!("0.0.0.0:{port}")).await?;

    Ok(())
}

async fn healthz(mut _req: Request<()>) -> tide::Result<String> {
    Ok("Ok".to_owned())
}

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    comcode: String,
    status: String,
}

async fn handle_get(req: Request<()>) -> tide::Result {
    let params = req.query::<Params>();
    match params {
        Err(e) => {
            println!("parse query params error");
            tide::Result::from(Err(e))
        }
        Ok(p) => {
            println!("comcode: {}, status: {}", p.comcode, p.status);
            let res = tide::Response::builder(200)
                .body(tide::Body::from_json(&p)?)
                .build();
            Ok(res)
        }
    }
}

async fn get_pon(req: Request<()>) -> tide::Result {
    let api_key = req.header("Ocp-Apim-Subscription-Key");
    if api_key.is_none() {
        let res = tide::Response::builder(401)
            .body("Unauthorized")
            .build();
        return Ok(res);
    }
    let key = api_key.unwrap();
    if key != "300db22ced354c639390bc0f9be9c943" {
        let res = tide::Response::builder(401)
            .body("Invalid API Key")
            .build();
        return Ok(res);
    }
    

    let mut rng = rand::thread_rng();
    match rng.gen_range(0..100) {
        0..=30 => {
            let res = tide::Response::builder(500)
                .body("Internal Server Error")
                .build();
            Ok(res)
        }
        _ => {
            let pons = get_pon_full();
            let last = rng.gen_range(0..pons.len());
            let pons_slice = &pons[0..=last];
            let res = tide::Response::builder(200)
                .body(tide::Body::from_json(&pons_slice)?)
                .build();
            Ok(res)
        }
    }
}
