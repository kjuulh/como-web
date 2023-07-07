use leptos::*;

use crate::features::dashboard_list_view::DashboardList;

#[component]
pub fn DashHomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="home-dash">
            <DashboardList/>
        </div>
    }
}
