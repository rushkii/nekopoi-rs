use slug::slugify;

use crate::request::request_api;
use crate::models;
use crate::utils::slug_web_url;


pub async fn get_recent() -> Result<models::Recent, reqwest::Error> {
    let res = request_api("/recent", None).await?;
    let mut data = res.json::<models::Recent>().await?;

    for carousel in &mut data.carousel {
        if !carousel.slug.is_none() {
            if let Some(slug) = &carousel.slug {
                carousel.web_url = Some(slug_web_url(slug.clone()));
            }
        }
    }

    for post in &mut data.posts {
        for p_data in &mut post.data {
            if !p_data.slug.is_none() {
                if let Some(slug) = &p_data.slug {
                    p_data.web_url = Some(slug_web_url(slug.clone()));
                }
            }
        }
    }

    Ok(data)
}

pub async fn search(query: &str, page: u64) -> Result<models::Search, reqwest::Error> {
    let params = [
        ("q", query),
        ("page", &page.to_string()),
    ];

    let res = request_api("/search", Some(&params)).await?;
    let mut data = res.json::<models::Search>().await?;

    for preview in &mut data.result {
        if !preview.slug.is_none() {
            if let Some(slug) = &preview.slug {
                preview.web_url = Some(slug_web_url(slug.clone()));
            }
        }
    }

    Ok(data)
}

pub async fn search_by_genre<T>(term_id: T) -> Result<models::Search, reqwest::Error>
where T: IntoIterator<Item = u64>
{
    let mut params: Vec<(&str, &str)> = Vec::new();

    for term in term_id {
        let term_str = term.to_string();
        let term_ref: &str = Box::leak(term_str.into_boxed_str());
        params.push(("term", term_ref));
    }

    let res = request_api("/searchByGenre", Some(&params)).await?;
    let mut data = res.json::<models::Search>().await?;

    for preview in &mut data.result {
        if !preview.slug.is_none() {
            if let Some(slug) = &preview.slug {
                preview.web_url = Some(slug_web_url(slug.clone()));
            }
        }
    }

    Ok(data)
}

pub async fn search_by_index(letter: &str, filter: &str) -> Result<models::Search, reqwest::Error> {
    let params = [
        ("letter", letter),
        ("type", filter),
        ("page", "1")
    ];

    let res = request_api("/listall", Some(&params)).await?;
    let mut data = res.json::<models::Search>().await?;

    for preview in &mut data.result {
        if preview.slug.is_none() {
            preview.slug = Some(slugify(preview.title.clone()));
            preview.web_url = Some(slug_web_url(preview.title.clone()));
        } else {
            if let Some(slug) = &preview.slug {
                preview.web_url = Some(slug_web_url(slug.clone()));
            }
        }
    }

    Ok(data)
}

pub async fn get_detail(id: u64) -> Result<models::Detail, reqwest::Error> {
    let res = request_api("/post", Some(&[("id", &id.to_string())])).await?;
    let mut data = res.json::<models::Detail>().await?;

    if let Some(slug) = data.slug.take() {
        let slg = slug;
        data.slug = Some(slg.clone());
        data.web_url = Some(slug_web_url(slg));
    }

    Ok(data)
}

pub async fn get_series(id: u64) -> Result<models::Series, reqwest::Error> {
    let res = request_api("/series", Some(&[("id", &id.to_string())])).await?;
    let mut data = res.json::<models::Series>().await?;

    if let Some(slug) = data.slug.take() {
        let slg = slug;
        data.slug = Some(slg.clone());
        data.web_url = Some(slug_web_url(slg));
    } else {
        data.slug = Some(slugify(data.title.clone()));
        data.web_url = Some(slug_web_url(data.title.clone()));
    }

    if let Some(episodes) = &mut data.episode {
        for eps in episodes {
            if eps.slug.is_none() {
                eps.slug = Some(slugify(eps.title.clone()));
                eps.web_url = Some(slug_web_url(eps.title.clone()));
            } else {
                if let Some(slug) = &eps.slug {
                    eps.web_url = Some(slug_web_url(slug.clone()));
                }
            }
        }
    }

    Ok(data)
}

pub async fn get_comments(slug: &str) -> Result<models::Discussion, reqwest::Error> {
    let res = request_api("/service/disqus/post", Some(&[("slug", &slug.to_string())])).await?;
    let data = res.json::<models::Discussion>().await?;

    Ok(data)
}

pub async fn get_coming_soon() -> Result<models::ComingSoon, reqwest::Error> {
    let res = request_api("/comingsoon", None).await?;
    let data = res.json::<models::ComingSoon>().await?;

    Ok(data)
}
