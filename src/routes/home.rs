use leptos::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row justify-between items-center bg-gray-800 p-4">
            <div class="flex flex-row items-center">
                <div class="text-2xl text-white font-bold">"Como - Todo"</div>
            </div>
            <div class="flex flex-row items-center space-x-4">
                <div class="text-xl text-white font-bold cursor-pointer">
                    <a href="http://localhost:3001/auth/zitadel?return_url=http://localhost:3000/dash/home">
                        "Enter"
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Navbar/>
        <h1 class="text-xl text-red-50">"Welcome to Leptos!"</h1>
    }
}
