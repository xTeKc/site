use perseus::Template;
use sycamore::prelude::{component, view, Html, SsrNode, View};

#[perseus::template(BlogPage)]
#[component(BlogPage<G>)]
pub fn blog_page() -> View<G> {
    view! {
        p { "Blog." }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Blog Page | Basic" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("blog").template(blog_page).head(head)
}
