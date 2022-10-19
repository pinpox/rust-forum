use vpub_rust::rocket_builder;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket_builder().launch().await?;
    Ok(())
}

