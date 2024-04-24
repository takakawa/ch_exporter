
use reqwest;
use actix_web::{ web, App, get, HttpServer,HttpResponse};
use config::{Config,FileFormat,File};
mod qry;


#[get("/metrics")]
async fn metrics(quries: web::Data<qry::Quries>) -> String {
    let client = reqwest::Client::new();
    let mut merge_resp = String::new();

    for     q in &quries.queries {

    
        let resp = client.get("http://127.0.0.1:8123")
        .header("x-clickhouse-user", "ck")
        .header("x-clickhouse-key", "M4XZ5biK")
        .query(&[("query",&q.query)])
        .send().await.unwrap().text().await.unwrap();
        merge_resp.push_str(&resp);

    }

    merge_resp

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut builder = Config::builder()

    .add_source(File::new("src/settings", FileFormat::Yaml)).build().unwrap();
  

    let config: qry::Quries = builder.try_deserialize().unwrap();
    println!("config: {:?}",config);

    let data = web::Data::new(config);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(metrics)
            
           
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}