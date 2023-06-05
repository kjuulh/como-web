use graphql_client::{GraphQLQuery, Response};
use leptos::*;
use leptos_router::*;

use crate::features::navbar_projects::gen::queries::get_projects_list_view::{
    ResponseData, Variables,
};
use crate::features::navbar_projects::gen::queries::GetProjectsListView;

use super::gen::queries::get_projects_list_view::GetProjectsListViewGetProjects;

pub async fn get_projects_list() -> anyhow::Result<Vec<GetProjectsListViewGetProjects>> {
    let request_body = GetProjectsListView::build_query(Variables {});
    let payload = serde_json::to_string(&request_body)?;
    let res = reqwasm::http::Request::post("http://localhost:3001/graphql")
        .credentials(reqwasm::http::RequestCredentials::Include)
        .body(payload)
        .send()
        .await?;
    let response_body: Response<ResponseData> = res.json().await?;
    Ok(response_body.data.unwrap().get_projects)
}

#[component]
pub fn NavbarProjectsView(
    cx: Scope,
    projects: Resource<(), Vec<GetProjectsListViewGetProjects>>,
) -> impl IntoView {
    let projects_view = move || {
        projects.with(cx, |projects| {

            if projects.is_empty() {
                return vec![view! { cx, <div class="project-item">"No projects"</div> }.into_any()];
            }

            projects
                .into_iter()
                .map(|project| {
                    view! { cx,
                        <a href=format!("/dash/project/{}", & project.id) class="project-item">
                            <div class="project-item-name hover:dark:bg-blue-700 rounded-md p-0.5 px-2">
                                {&project.name}
                            </div>
                        </a>
                    }.into_any()
                })
                .collect::<Vec<_>>()
        })
    };

    view! { cx, <div class="project-items space-y-1">{projects_view}</div> }
}

#[component]
pub fn NavbarProjects(cx: Scope) -> impl IntoView {
    let projects =
        create_local_resource(cx, || (), |_| async { get_projects_list().await.unwrap() });

    view! { cx, <NavbarProjectsView projects=projects/> }
}
