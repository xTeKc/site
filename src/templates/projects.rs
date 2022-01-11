// update "greeting" to something else

use perseus::{
    http::header::{HeaderMap, HeaderName},
    Html, RenderFnResultWithCause, SsrNode, Template,
};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, view, View};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectsPageProps {
    pub greeting: String,
}

#[perseus::template(ProjectsPage)]
#[component(ProjectsPage<G>)]
pub fn projects_page(props: ProjectsPageProps) -> View<G> {
    view! {
        p {(props.greeting)}
        a(href = "index", id = "index-link") { "Home!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("projects")
        .build_state_fn(get_build_props)
        .template(projects_page)
        .head(head)
        .set_headers_fn(set_headers)
}

#[perseus::head]
pub fn head(_props: ProjectsPageProps) -> View<SsrNode> {
    view! {
        title { "Projects Page | Web App" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<ProjectsPageProps> {
    Ok(ProjectsPageProps {
        greeting: "Hello from Projects page!".to_string(),
    })
}

#[perseus::autoserde(set_headers)]
pub fn set_headers(props: Option<ProjectsPageProps>) -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert(
        HeaderName::from_lowercase(b"x-greeting").unwrap(),
        props.unwrap().greeting.parse().unwrap(),
    );
    map
}

