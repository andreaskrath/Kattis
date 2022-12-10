pub struct Solution {
    pub name: String,
    link: String,
    language: String,
    path: String,
}

impl Solution {
    pub fn new(name: String, link: String, language: String, path: String) -> Self {
        Self {
            name,
            link,
            language,
            path,
        }
    }

    pub fn to_table_string(&self) -> String {
        format!(
            "|[{}]({})|[{}]({})|\n",
            self.name, self.link, self.language, self.path
        )
    }
}
