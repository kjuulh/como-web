#![allow(clippy::all, warnings)]
pub struct GetProjectsListView;
pub mod get_projects_list_view {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetProjectsListView";
    pub const QUERY: &str =
        "query GetProjectsListView {\n  getProjects {\n    id\n    name\n  }\n}\n";
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type UUID = crate::common::graphql::UUID;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "getProjects")]
        pub get_projects: Vec<GetProjectsListViewGetProjects>,
    }
    #[derive(Deserialize)]
    pub struct GetProjectsListViewGetProjects {
        pub id: UUID,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for GetProjectsListView {
    type Variables = get_projects_list_view::Variables;
    type ResponseData = get_projects_list_view::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_projects_list_view::QUERY,
            operation_name: get_projects_list_view::OPERATION_NAME,
        }
    }
}
