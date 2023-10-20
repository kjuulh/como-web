use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::common::layout::DashboardLayout;
use crate::routes::dash::home::DashHomePage;
use crate::routes::features_view::FeaturesView;
use crate::routes::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/como_web.css"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|| {
                            view! { <HomePage/> }
                        }
                    />
                    <Route
                        path="/dash"
                        view=|| {
                            view! { <DashboardLayout/> }
                        }
                    >
                        <Route
                            path=""
                            view=|| {
                                view! { <DashHomePage/> }
                            }
                        />
                        <Route
                            path="home"
                            view=|| {
                                view! { <DashHomePage/> }
                            }
                        />
                    </Route>
                    <Route
                        path="/features"
                        view=|| {
                            view! { <FeaturesView/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
