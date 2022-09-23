use gloo::net::http::Request;

pub(crate) struct Resources(String);

impl Resources {
    pub(crate) fn from_file_name(file_name: &str) -> Self {
        Self(format!("static/{}", file_name))
    }

    pub(crate) fn from_path(path: &str) -> Self {
        Self(format!("{}", path))
    }

    pub(crate) async fn request_string(&self) -> Result<String, anyhow::Error> {
        Ok(Request::get(&self.0)
            .header("responseType", "blob")
            .send()
            .await?
            .text()
            .await?)
    }

    pub(crate) async fn request_binary(&self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(Request::get(&self.0)
            .header("responseType", "blob")
            .send()
            .await?
            .binary()
            .await?)
    }
}
