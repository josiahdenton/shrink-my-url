pub trait SaveMinification {
    // save the alias to the url mapping
    // if the mapping already existed before, return true
    fn save(&mut self, alias: &str, url: &str) -> anyhow::Result<()>;
    // TODO: may need to fix this to return a result
}

pub trait EncodeUrl {
    fn encode(&self, alias: &str) -> String;
}

#[derive(Debug, Clone)]
pub struct Minifier<R, E> {
    repository: R,
    encoder: E,
}

impl<R, E> Minifier<R, E>
where
    R: SaveMinification,
    E: EncodeUrl,
{
    pub fn new(repository: R, encoder: E) -> Self {
        Self {
            repository,
            encoder,
        }
    }

    pub fn minify(&mut self, url: &str) -> anyhow::Result<String> {
        let mut alias = self.encoder.encode(url);
        while let Err(_) = self.repository.save(&alias, url) {
            alias = self.encoder.encode(url);
        }
        Ok(alias)
    }
}
