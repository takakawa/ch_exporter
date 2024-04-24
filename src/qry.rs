#[derive(Debug, Default, serde::Deserialize, Clone)]
pub struct Query {
    pub query: String,
}

#[derive(Debug, Default, serde::Deserialize, Clone)]
pub struct Quries {
    pub queries: Vec<Query>,
}
