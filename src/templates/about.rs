// update "greeting" to something else

use perseus::{
    http::header::{HeaderMap, HeaderName},
    Html, RenderFnResultWithCause, SsrNode, Template,
};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, view, View};

#[derive(Serialize, Deserialize, Debug)]
pub struct AboutPageProps {
    pub greeting: String,
}

#[perseus::template(AboutPage)]
#[component(AboutPage<G>)]
pub fn about_page(props: AboutPageProps) -> View<G> {
    view! {
        p {(props.greeting)}
        a(href = "index", id = "index-link") { "Home!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about")
        .build_state_fn(get_build_props)
        .template(about_page)
        .head(head)
        .set_headers_fn(set_headers)
}

#[perseus::head]
pub fn head(_props: AboutPageProps) -> View<SsrNode> {
    view! {
        title { "About Page | Web App" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<AboutPageProps> {
    Ok(AboutPageProps {
        greeting: "Hello from About page!".to_string(),
    })
}

#[perseus::autoserde(set_headers)]
pub fn set_headers(props: Option<AboutPageProps>) -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert(
        HeaderName::from_lowercase(b"x-greeting").unwrap(),
        props.unwrap().greeting.parse().unwrap(),
    );
    map
}