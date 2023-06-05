use leptos::*;
use uuid::Uuid;

use crate::features::navbar_projects::gen::queries::get_projects_list_view::GetProjectsListViewGetProjects;
use crate::features::navbar_projects::NavbarProjectsView;

#[component]
pub fn FeaturesView(cx: Scope) -> impl IntoView {
    let projects = create_local_resource(
        cx,
        || (),
        |_| async {
            vec![
                GetProjectsListViewGetProjects {
                    id: Uuid::new_v4(),
                    name: "some-name".to_string(),
                },
                GetProjectsListViewGetProjects {
                    id: Uuid::new_v4(),
                    name: "some-other-name".to_string(),
                },
            ]
        },
    );

    let emptyProjects: Resource<(), Vec<GetProjectsListViewGetProjects>> =
        create_local_resource(cx, || (), |_| async { Vec::new() });

    view! { cx,
        <div>
            <div class="space-y-5">
                <h1>"NavbarProjects"</h1>
                <h2>"Projects"</h2>
                <div class="feature-case">
                    <NavbarProjectsView projects=projects/>
                </div>
                <h2>"no projects"</h2>
                <div class="feature-case">
                    <NavbarProjectsView projects=emptyProjects/>
                </div>
            </div>
        </div>
    }
}
