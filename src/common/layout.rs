use leptos::*;
use leptos_router::*;

use crate::features::command_line::CommandLine;
use crate::features::navbar_projects::NavbarProjects;

#[component]
pub fn DashNav() -> impl IntoView {
    view! {
        <nav class="min-w-[200px] p-4 space-y-4 h-screen sticky top-0 select-none bg-gray-800">
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
pub fn DashboardLayout() -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <DashNav/>
            <div id="content" class="px-0.5 flex-grow">
                <CommandLine>
                    <Outlet/>
                </CommandLine>
            </div>
        </div>
    }
}
