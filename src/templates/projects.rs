use perseus::Template;
use sycamore::prelude::{component, view, Html, SsrNode, View};

#[perseus::template(ProjectsPage)]
#[component(ProjectsPage<G>)]
pub fn projects_page() -> View<G> {
    view! {
        p { "Projects." }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Projects Page | Basic" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("projects").template(projects_page).head(head)
}
