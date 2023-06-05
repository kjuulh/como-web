use leptos::*;
use leptos_router::*;

use crate::features::navbar_projects::NavbarProjects;

#[component]
pub fn DashNav(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav class="absolute min-w-[200px] p-4 space-y-4 h-screen sticky top-0 select-none">
            <div>
                <a href="/dash/home" class="text-xl">
                    "como"
                </a>
            </div>
            <div>
                <a href="/dash/current" class="">
                    "inbox"
                </a>
            </div>
            <div>
                <p class="text-sm mb-0.5 dark:text-gray-500">"Favorites"</p>
                <a href="/dash/current" class="dark:text-gray-300 pl-2">
                    "inbox"
                </a>
            </div>
            <div>
                <p class="text-sm mb-0.5 dark:text-gray-500">"Projects"</p>
                <div class="pl-2 dark:text-gray-300">
                    <NavbarProjects/>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn DashboardLayout(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row">
            <DashNav/>
            <div id="content" class="p-2">
                <Outlet/>
            </div>
        </div>
    }
}
