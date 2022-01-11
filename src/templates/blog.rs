// update "greeting" to something else

use perseus::{
    http::header::{HeaderMap, HeaderName},
    Html, RenderFnResultWithCause, SsrNode, Template,
};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, view, View};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPageProps {
    pub greeting: String,
}

#[perseus::template(BlogPage)]
#[component(BlogPage<G>)]
pub fn blog_page(props: BlogPageProps) -> View<G> {
    view! {
        p {(props.greeting)}
        a(href = "index", id = "index-link") { "Home!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("blog")
        .build_state_fn(get_build_props)
        .template(blog_page)
        .head(head)
        .set_headers_fn(set_headers)
}

#[perseus::head]
pub fn head(_props: BlogPageProps) -> View<SsrNode> {
    view! {
        title { "Blog Page | Web App" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<BlogPageProps> {
    Ok(BlogPageProps {
        greeting: "Hello from Blog page!".to_string(),
    })
}

#[perseus::autoserde(set_headers)]
pub fn set_headers(props: Option<BlogPageProps>) -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert(
        HeaderName::from_lowercase(b"x-greeting").unwrap(),
        props.unwrap().greeting.parse().unwrap(),
    );
    map
}
