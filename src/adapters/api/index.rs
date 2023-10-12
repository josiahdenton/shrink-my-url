use crate::domain::templates::IndexTemplate;

const NAME: &'static str = "Josiah";

pub async fn serve_index() -> IndexTemplate<'static> {
    log::info!("get for index!");
    IndexTemplate::new(NAME)
}
