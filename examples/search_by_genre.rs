use nekopoi::utils;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = nekopoi::search_by_genre(vec![36, 37]).await?;
    println!("{}", utils::prettify(&res).unwrap());
    Ok(())
}
