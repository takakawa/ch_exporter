use actix_web::{get, web, App, HttpServer};
use config::{Config, File, FileFormat};
use reqwest;
mod qry;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
   
    #[arg(long,default_value ="config/queries.yaml",env="QUERY_FILE")]
    query_file: String,

    #[arg(long,default_value ="http://127.0.0.1:8123",env="CH_HOST")]
    ch_host: String,

       
    #[arg( long,env="CH_USER")]
    ch_user: String,

    #[arg( long,env="CH_PASSWORD")]
    ch_password: String,

    #[arg(long, default_value_t = 8080,env="METRIC_PORT")]
    metric_port : u16,


}

struct Cfg {
    q : qry::Quries,
    ch_host : String,
    ch_user : String,
    ch_password: String
}

#[get("/metrics")]
async fn metrics(cfg: web::Data<Cfg>) -> String {
    let client = reqwest::Client::new();
    let mut merge_resp = String::new();

    for q in &cfg.q.queries {
        let resp = client
            .get(&cfg.ch_host)
            .header("x-clickhouse-user", &cfg.ch_user)
            .header("x-clickhouse-key", &cfg.ch_password)
            .query(&[("query", &q.query)])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        merge_resp.push_str(&resp);
    }

    merge_resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let  builder = Config::builder()
        .add_source(File::new(&args.query_file, FileFormat::Yaml))
        .build()
        .unwrap();

    let config: qry::Quries = builder.try_deserialize().unwrap();
  

    let data = web::Data::new(Cfg{
            q :config,
            ch_host: args.ch_host,
            ch_password:args.ch_password,
        ch_user:args.ch_user});
    HttpServer::new(move || App::new().app_data(data.clone()).service(metrics))
        .bind(("0.0.0.0", args.metric_port))?
        .run()
        .await
}
