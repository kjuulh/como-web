use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::common::layout::DashboardLayout;
use crate::routes::dash::home::DashHomePage;
use crate::routes::features_view::FeaturesView;
use crate::routes::home::HomePage;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/como_web.css"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <HomePage/> }
                        }
                    />
                    <Route
                        path="/dash"
                        view=|cx| {
                            view! { cx, <DashboardLayout/> }
                        }
                    >
                        <Route
                            path=""
                            view=|cx| {
                                view! { cx, <DashHomePage/> }
                            }
                        />
                        <Route
                            path="home"
                            view=|cx| {
                                view! { cx, <DashHomePage/> }
                            }
                        />
                    </Route>
                    <Route
                        path="/features"
                        view=|cx| {
                            view! { cx, <FeaturesView/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
