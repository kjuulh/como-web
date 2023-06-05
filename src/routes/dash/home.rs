use leptos::*;

use crate::features::navbar_projects::NavbarProjects;

#[component]
pub fn DashHomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="home-dash">
            <NavbarProjects/>
        </div>
    }
}
