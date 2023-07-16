pub fn slug(text: String) -> String {
    slug::slugify(text)
}

pub fn slug_web_url(text: String) -> String {
    "https://nekopoi.care/".to_string() + slug(text).as_str()
}

pub fn prettify<T>(res: &T) -> Result<String, serde_json::Error>
where T: serde::Serialize
{
	serde_json::to_string_pretty(&res)
}
