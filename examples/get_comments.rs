use nekopoi::utils;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = nekopoi::get_comments("shoujo-ramune-episode-3-subtitle-indonesia").await?;
    println!("{}", utils::prettify(&res).unwrap());
    Ok(())
}
