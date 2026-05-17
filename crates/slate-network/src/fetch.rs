//! The streaming fetcher.
//!
//! `Fetcher::fetch_streaming` returns an async stream of `Bytes`
//! chunks. The caller typically pipes each chunk through an
//! incremental parser and forwards the emitted `OwnedWebCall`s
//! to the kernel, so the first paint can occur well before the page
//! is fully downloaded.

use bytes::Bytes;
use futures_util::Stream;
use reqwest::Client;

use crate::sandbox::OriginPolicy;

#[derive(Debug)]
pub enum FetchError {
    OriginDenied,
    Http(reqwest::Error),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::OriginDenied => write!(f, "origin denied by sandbox policy"),
            FetchError::Http(e)      => write!(f, "http error: {e}"),
        }
    }
}

impl std::error::Error for FetchError {}

impl From<reqwest::Error> for FetchError {
    fn from(e: reqwest::Error) -> Self { FetchError::Http(e) }
}

pub struct Fetcher {
    client: Client,
    policy: OriginPolicy,
}

impl Fetcher {
    pub fn new(policy: OriginPolicy) -> Result<Self, FetchError> {
        let client = Client::builder()
            .use_rustls_tls()
            .build()?;
        Ok(Self { client, policy })
    }

    /// Fetch the URL and return a chunked byte stream. Streams chunks
    /// as they arrive on the wire — critical for progressive paint.
    pub async fn fetch_streaming(
        &self,
        url: &str,
    ) -> Result<impl Stream<Item = reqwest::Result<Bytes>>, FetchError> {
        if !self.policy.permits(url) {
            return Err(FetchError::OriginDenied);
        }
        let resp = self.client.get(url).send().await?;
        Ok(resp.bytes_stream())
    }

    /// Convenience: fetch the whole body. Non-streaming callers only.
    pub async fn fetch_all(&self, url: &str) -> Result<Bytes, FetchError> {
        if !self.policy.permits(url) {
            return Err(FetchError::OriginDenied);
        }
        let resp = self.client.get(url).send().await?;
        Ok(resp.bytes().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_denied_without_entry() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let f = Fetcher::new(OriginPolicy::allow_list(["https://a.example"])).unwrap();
            let err = f.fetch_all("https://b.example/").await;
            assert!(matches!(err, Err(FetchError::OriginDenied)));
        });
    }
}
