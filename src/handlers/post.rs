use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub author: String,
    pub title: String,
    pub url: String,
    pub permalink: String,
    pub image_url: String,
}

impl Post {
    pub fn new(
        author: String,
        title: String,
        url: String,
        permalink: String,
        image_url: String,
    ) -> Self {
        Post {
            author,
            title,
            url,
            permalink,
            image_url,
        }
    }
}
