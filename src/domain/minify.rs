pub enum SaveResult {
    UrlExists,
    UrlAdded,
}

pub trait SaveMinification {
    // save the alias to the url mapping 
    // if the mapping already existed before, return true 
    fn save(&mut self, alias: String, url: String) -> SaveResult;
    // TODO: may need to fix this to return a result
}

pub trait EncodeUrl {
    fn encode(&self, alias: &str) -> String;
}

#[derive(Debug)]
pub struct MinifyRequest {
    url: String,
    alias: String,
}

impl MinifyRequest {
    pub fn new(url: String, alias: String) -> Self {
        // TODO: add url verification
        Self { url, alias }
    }
}

#[derive(Debug)]
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

    pub fn minify(&mut self, minify_request: MinifyRequest) {
        if minify_request.alias.len() == 0 {
            let alias = self.encoder.encode(&minify_request.url);
            let result = self.repository.save(alias, minify_request.url);
        } else {
            self.repository.save(minify_request.alias, minify_request.url);
        }
    }
}
