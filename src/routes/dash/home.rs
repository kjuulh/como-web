use leptos::*;

use crate::features::dashboard_list_view::DashboardList;

#[component]
pub fn DashHomePage() -> impl IntoView {
    view! {
        <div class="home-dash">
            <DashboardList/>
        </div>
    }
}
