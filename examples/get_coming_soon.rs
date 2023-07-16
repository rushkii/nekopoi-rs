use nekopoi::utils;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = nekopoi::get_coming_soon().await?;
    println!("{}", utils::prettify(&res).unwrap());
    Ok(())
}
