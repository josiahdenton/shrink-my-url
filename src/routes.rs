use crate::templates::IndexTemplate;

const NAME: &'static str = "Josiah";

pub async fn index() -> IndexTemplate<'static> {
   IndexTemplate::new(NAME)
}

pub async fn click() -> String {
    String::from("<p>You clicked me!</p>")
}
