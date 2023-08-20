use graphql_client::{GraphQLQuery, Response};
use leptos::*;

use crate::features::dashboard_list_view::gen::queries::get_projects_list_view::GetProjectsListViewGetProjectsItems;

use super::gen::queries::get_projects_list_view::{
    GetProjectsListViewGetProjects, ResponseData, Variables,
};
use super::gen::queries::GetProjectsListView;

pub async fn get_projects_list() -> anyhow::Result<Vec<GetProjectsListViewGetProjects>> {
    let request_body = GetProjectsListView::build_query(Variables {});
    let payload = serde_json::to_string(&request_body)?;
    let res = reqwasm::http::Request::post("http://localhost:3001/graphql")
        .credentials(reqwasm::http::RequestCredentials::Include)
        .body(payload)
        .send()
        .await?;
    let response_body: Response<ResponseData> = res.json().await?;
    Ok(response_body
        .data
        .ok_or(anyhow::anyhow!("failed to get projects list"))?
        .get_projects)
}

#[component]
pub fn DashboardItemView(cx: Scope, item: GetProjectsListViewGetProjectsItems) -> impl IntoView {
    view! { cx, <div class="dashboard-list-item dashboard-item">{item.title}</div> }
}

#[component]
pub fn DashboardProjectItemView(
    cx: Scope,
    project: GetProjectsListViewGetProjects,
) -> impl IntoView {
    view! { cx,
        <div class="dashboard-list-item dashboard-list-project">
            <a href=format!("/dash/project/{}", & project.id) class="project-item flex flex-row">
                <div class="space-x-2">
                    <span>{&project.name}</span>
                    <span class="text-gray-50">{project.items.len()}</span>
                    <span class="flex-grow"></span>
                </div>
            </a>
        </div>
    }
}

#[component]
pub fn DashboardListView(
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
                .filter(|project| !project.items.is_empty())
                .map(|project| {
                    view! { cx,
                        <div>
                            <DashboardProjectItemView project=project.clone()/>
                            {&project
                                .items
                                .clone()
                                .into_iter()
                                .map(|item| {
                                    view! { cx, <DashboardItemView item=item/> }
                                })
                                .collect::<Vec<_>>()
                                .into_view(cx)}
                        </div>
                    }
                    .into_any()
                })
                .collect::<Vec<_>>()
        })
    };

    view! { cx, <div class="project-items">{projects_view}</div> }
}

#[component]
pub fn DashboardList(cx: Scope) -> impl IntoView {
    let projects =
        create_local_resource(cx, || (), |_| async { get_projects_list().await.unwrap() });

    view! { cx, <DashboardListView projects=projects/> }
}
