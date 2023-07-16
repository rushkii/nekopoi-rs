use nekopoi::utils;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = nekopoi::search_by_index("0-9", "3d_hentai").await?;
    println!("{}", utils::prettify(&res).unwrap());
    Ok(())
}
