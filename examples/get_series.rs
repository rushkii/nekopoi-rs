use nekopoi::utils;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = nekopoi::get_series(3012).await?;
    println!("{}", utils::prettify(&res).unwrap());
    Ok(())
}
