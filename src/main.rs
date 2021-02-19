use color_eyre::Result;
use test_actix::run;

#[actix_web::main]
async fn main() -> Result<()> {
    run().await
}
