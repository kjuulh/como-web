use leptos::{ev, html::*, *};
use uuid::Uuid;

use crate::features::navbar_projects::gen::queries::get_projects_list_view::GetProjectsListViewGetProjects;
use crate::features::navbar_projects::{
    NavbarProjects, NavbarProjectsProps, NavbarProjectsView, NavbarProjectsViewProps,
};

pub fn features_view(cx: Scope) -> impl IntoView {
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

    return div(cx).child(
        div(cx)
            .classes("space-y-5 p-2")
            .child(h1(cx).child("NavbarProjects"))
            .child(h2(cx).child("Projects"))
            .child(
                div(cx)
                    .classes("feature-case")
                    .child(NavbarProjectsView(cx, NavbarProjectsViewProps { projects })),
            )
            .child(h2(cx).child("no projects"))
            .child(div(cx).classes("feature-case").child(NavbarProjectsView(
                cx,
                NavbarProjectsViewProps {
                    projects: emptyProjects,
                },
            ))),
    );
}
