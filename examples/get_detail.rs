use nekopoi::utils;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = nekopoi::get_detail(6995).await?;
    println!("{}", utils::prettify(&res).unwrap());
    Ok(())
}
