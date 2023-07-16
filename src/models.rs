use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct RecentData {
    pub category: String,
    pub data: Vec<Preview>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Preview {
    pub id: u64,
    pub r#type: Option<String>,
    pub date: String,
    pub title: String,
    pub image: String,
    pub description: Option<String>,
    pub slug: Option<String>,
    pub web_url: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    name: Option<String>,
    link: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Download {
    r#type: String,
    links: Vec<Link>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Genre {
    term_id: u64,
    name: String,
    slug: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeriesPreview {
    id: u64,
    title: String,
    content: String,
    image: String,
    genre: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoMeta {
    aliases: String,
    episode: String,
    status: String,
    durasi: String,
    skor: String,
    produser: String,
    tayang: String,
    genre: Option<Vec<Genre>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorComment {
    id: String,
    username: String,
    name: String,
    avatar: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LikeComment {
    likes: u64,
    dislikes: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageComment {
    html: String,
    raw: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    id: String,
    author: AuthorComment,
    like: LikeComment,
    message: MessageComment,
    created_at: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Discussion {
    thread: String,
    #[serde(rename="hasNext")]
    has_next: bool,
    #[serde(rename="hasPrev")]
    has_previous: bool,
    total: u64,
    result: Option<Vec<Comment>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recent {
    pub carousel: Vec<Preview>,
    pub posts: Vec<RecentData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    pub result: Vec<Preview>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Detail {
    pub id: u64,
    pub date: String,
    pub title: String,
    pub image: String,
    pub content: String,
    pub series: Option<SeriesPreview>,
    pub note: Option<String>,
    pub slug: Option<String>,
    pub web_url: Option<String>,
    pub stream: Vec<Link>,
    pub download: Option<Vec<Download>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Series {
    pub id: u64,
    pub date: String,
    pub title: String,
    pub image: String,
    pub slug: Option<String>,
    pub web_url:Option<String>,
    pub info_meta: InfoMeta,
    pub episode: Option<Vec<Preview>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComingSoon {
    pub result: String
}
