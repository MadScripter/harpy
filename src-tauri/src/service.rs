use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, IntoHeaderName},
    Client, Method, RequestBuilder,
};
use serde::Serialize;

pub struct Service {
    base_url: String,
    client: Client,
    auth_token: Option<String>,
    headers: HeaderMap,
}

impl Service {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            headers: HeaderMap::new(),
            auth_token: None,
        }
    }

    pub fn set_token(&mut self, token: &str) {
        self.auth_token = Some(token.into());
    }

    pub fn clear_token(&mut self) {
        self.auth_token = None;
    }

    pub fn with_header<K: IntoHeaderName>(&mut self, key: K, value: &str) -> &mut Self {
        let header_value = HeaderValue::from_str(value).unwrap();

        self.headers.insert(key, header_value);

        self
    }

    pub fn with_auth(&mut self) -> &mut Self {
        let value = format!("Bearer {}", self.auth_token.as_ref().unwrap());

        let mut header_value = HeaderValue::from_str(&value).unwrap();
        header_value.set_sensitive(true);

        self.headers.insert("Authorization", header_value);

        self
    }

    pub fn with_character(&mut self, character_id: &str) -> &mut Self {
        self.with_header("X-Authenticated-Character", character_id)
    }

    pub fn with_session(&mut self) -> &mut Self {
        self.with_header("X-Session-Id", "GameSession")
    }

    pub fn with_version(&mut self, version: &str) -> &mut Self {
        self.with_header("X-Client-Version", version)
    }

    fn create_request(&mut self, method: Method, path: &str) -> RequestBuilder {
        let url = format!("{}/{}", self.base_url, path);

        let request = self
            .client
            .request(method, url)
            .headers(self.headers.clone());

        self.headers.clear();

        request
    }

    pub fn get(&mut self, path: &str) -> RequestBuilder {
        self.create_request(Method::GET, path)
    }

    pub fn query<T: Serialize + ?Sized>(&mut self, path: &str, params: &T) -> RequestBuilder {
        self.create_request(Method::GET, path).query(params)
    }

    pub fn post<T: Serialize + ?Sized>(&mut self, path: &str, body: Option<&T>) -> RequestBuilder {
        let request = self.create_request(Method::POST, path);

        if let Some(data) = body {
            return request.json(data);
        }

        request
    }
}
