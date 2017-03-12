use clubdarn;
use error::*;
use reqwest;
use reqwest::header::{Authorization, Basic};
use serde_json;

pub struct Client {
    http: reqwest::Client,
    base_url: String,
    index: String,
}

impl Client {
    pub fn new(base_url: String, index: String) -> Result<Self> {
        let http = reqwest::Client::new().chain_err(|| "Could not create Elasticsearch client")?;

        Ok(Client {
            http: http,
            base_url: base_url,
            index: index,
        })
    }

    pub fn search_series(&self, query: &str) -> Result<Vec<clubdarn::Series>> {
        let query_json = json!({
            "query": {
                "multi_match": {
                    "query": query,
                    "fields": SERIES_SEARCH_FIELDS
                }
            }
        });

        let url_str = format!("{}/{}/_search", self.base_url, self.index);
        let url = reqwest::Url::parse(&url_str).chain_err(|| format!("Invalid url {}", url_str))?;

        let auth = if !url.username().is_empty() || !url.password().is_none() {
            Some(Authorization(Basic {
                username: url.username().to_string(),
                password: url.password().map(|p| p.to_string()),
            }))
        } else {
            None
        };

        let mut request = self.http
            .post(url)
            .json(&query_json);

        if let Some(a) = auth {
            request = request.header(a);
        }

        let mut result_json = request.send()
            .chain_err(|| "Search failed")?
            .json::<serde_json::Value>()
            .chain_err(|| "JSON deserialization failed")?;

        let mut empty_vec = Vec::new();
        let hits = result_json.pointer_mut("/hits/hits")
            .and_then(|r| r.as_array_mut())
            .unwrap_or(&mut empty_vec);

        let series = hits.into_iter()
            .flat_map(|hit| hit.pointer_mut("/_source/titles/clubdam"))
            .flat_map(|titles| titles.as_array_mut())
            .flat_map(|titles_opt| titles_opt)
            .flat_map(|title| {
                title.as_str().map(|t| {
                    clubdarn::Series {
                        title: t.to_string(),
                        first_kana: None,
                    }
                })
            })
            .collect::<Vec<_>>();

        Ok(series)
    }
}

const SERIES_SEARCH_FIELDS: [&'static str; 4] =
    ["titles.x-jat", "titles.ja", "titles.en", "titles.clubdam"];
