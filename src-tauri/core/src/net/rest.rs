use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
    Client,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSettings {
    pub api: String,
    pub cdn: String,
    pub gateway: String,
    pub wellknown: String,
}

impl Default for RouteSettings {
    fn default() -> Self {
        Self {
            api: "https://api.old.server.spacebar.chat/api".to_string(),
            cdn: "https://cdn.old.server.spacebar.chat".to_string(),
            gateway: "wss://gateway.old.server.spacebar.chat".to_string(),
            wellknown: "https://spacebar.chat".to_string(),
        }
    }
}

fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Spacebar-Client/1.0"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers
}

#[derive(Clone)]
pub struct RestClient {
    pub route_settings: RouteSettings,
    client: Client,
    headers: HeaderMap,
}

#[derive(Debug, Error)]
pub enum RestError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error("api error: {0}")]
    Api(String),
}

#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    message: String,
}

impl RestClient {
    pub fn new(route_settings: RouteSettings) -> Self {
        let client = Client::new();
        let headers = default_headers();
        Self {
            route_settings,
            client,
            headers,
        }
    }

    pub fn set_token(&mut self, token: Option<&str>) {
        if let Some(token) = token {
            if let Ok(value) = HeaderValue::from_str(token) {
                self.headers.insert("Authorization", value);
            }
        } else {
            self.headers.remove("Authorization");
        }
    }

    pub async fn get_endpoints_from_domain(url: Url) -> Result<RouteSettings, RestError> {
        match Self::get_instance_domains(&url, &url).await {
            Ok(settings) => Ok(settings),
            Err(_) => {
                let client = Client::new();
                let well_known_url = format!(
                    "{}/.well-known/spacebar",
                    url.origin().ascii_serialization()
                );
                #[derive(Deserialize)]
                struct WellKnownResponse {
                    api: String,
                }
                let res: WellKnownResponse = client
                    .get(&well_known_url)
                    .headers(default_headers())
                    .send()
                    .await?
                    .json()
                    .await?;
                let api_url = Url::parse(&res.api)?;
                Self::get_instance_domains(&api_url, &url).await
            }
        }
    }

    pub async fn get_instance_domains(
        url: &Url,
        knownas: &Url,
    ) -> Result<RouteSettings, RestError> {
        let mut base = url.clone();
        if !base.path().contains("api") {
            base.path_segments_mut().expect("valid url").push("api");
        }
        let endpoint = base.join("policies/instance/domains").unwrap();
        #[derive(Deserialize)]
        struct InstanceDomainsResponse {
            #[serde(rename = "apiEndpoint")]
            api_endpoint: String,
            gateway: String,
            cdn: String,
        }
        let client = Client::new();
        let res: InstanceDomainsResponse = client
            .get(endpoint)
            .headers(default_headers())
            .send()
            .await?
            .json()
            .await?;
        Ok(RouteSettings {
            api: res.api_endpoint,
            gateway: res.gateway,
            cdn: res.cdn,
            wellknown: knownas.to_string(),
        })
    }

    pub fn make_api_url(&self, path: &str, query: &[(&str, &str)]) -> Url {
        let mut url = Url::parse(&(self.route_settings.api.clone() + path)).unwrap();
        for (k, v) in query {
            url.query_pairs_mut().append_pair(k, v);
        }
        url
    }

    pub fn make_cdn_url(&self, path: &str, query: &[(&str, &str)]) -> Url {
        let mut url = Url::parse(&(self.route_settings.cdn.clone() + path)).unwrap();
        for (k, v) in query {
            url.query_pairs_mut().append_pair(k, v);
        }
        url
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, RestError> {
        let url = self.make_api_url(path, query);
        let res = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?;
        Self::parse_response(res).await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<&B>,
        query: &[(&str, &str)],
        headers: &[(&str, &str)],
    ) -> Result<T, RestError> {
        let url = self.make_api_url(path, query);
        let mut req = self.client.post(url).headers(self.headers.clone());
        for (k, v) in headers {
            req = req.header(*k, *v);
        }
        if let Some(b) = body {
            req = req.json(b);
        }
        let res = req.send().await?;
        Self::parse_response(res).await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<&B>,
        query: &[(&str, &str)],
        headers: &[(&str, &str)],
    ) -> Result<T, RestError> {
        let url = self.make_api_url(path, query);
        let mut req = self.client.put(url).headers(self.headers.clone());
        for (k, v) in headers {
            req = req.header(*k, *v);
        }
        if let Some(b) = body {
            req = req.json(b);
        }
        let res = req.send().await?;
        Self::parse_response(res).await
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<&B>,
        query: &[(&str, &str)],
        headers: &[(&str, &str)],
    ) -> Result<T, RestError> {
        let url = self.make_api_url(path, query);
        let mut req = self.client.patch(url).headers(self.headers.clone());
        for (k, v) in headers {
            req = req.header(*k, *v);
        }
        if let Some(b) = body {
            req = req.json(b);
        }
        let res = req.send().await?;
        Self::parse_response(res).await
    }

    pub async fn post_form_data<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
        query: &[(&str, &str)],
        headers: &[(&str, &str)],
    ) -> Result<T, RestError> {
        let url = self.make_api_url(path, query);
        let mut req = self
            .client
            .post(url)
            .headers(self.headers.clone())
            .multipart(form);
        for (k, v) in headers {
            req = req.header(*k, *v);
        }
        let res = req.send().await?;
        Self::parse_response(res).await
    }

    pub async fn delete(
        &self,
        path: &str,
        query: &[(&str, &str)],
        headers: &[(&str, &str)],
    ) -> Result<(), RestError> {
        let url = self.make_api_url(path, query);
        let mut req = self.client.delete(url).headers(self.headers.clone());
        for (k, v) in headers {
            req = req.header(*k, *v);
        }
        let res = req.send().await?;
        if res.status().is_success() {
            Ok(())
        } else {
            let text = res.text().await?;
            if let Ok(err) = serde_json::from_str::<ApiErrorResponse>(&text) {
                Err(RestError::Api(err.message))
            } else {
                Err(RestError::Api(text))
            }
        }
    }

    async fn parse_response<T: DeserializeOwned>(
        res: reqwest::Response,
    ) -> Result<T, RestError> {
        if res.status().is_success() {
            Ok(res.json::<T>().await?)
        } else {
            let text = res.text().await?;
            if let Ok(err) = serde_json::from_str::<ApiErrorResponse>(&text) {
                Err(RestError::Api(err.message))
            } else {
                Err(RestError::Api(text))
            }
        }
    }
}
