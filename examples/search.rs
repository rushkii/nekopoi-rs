use nekopoi::utils;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = nekopoi::search("shoujo ramune", 1).await?;
    println!("{}", utils::prettify(&res).unwrap());
    Ok(())
}
