#![allow(clippy::all, warnings)]
pub struct GetProjectsListView;
pub mod get_projects_list_view {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetProjectsListView";
    pub const QUERY : & str = "query GetProjectsListView {\n  getProjects {\n    id\n    name\n\n    items {\n      id\n      title\n      description\n      state\n    }\n  }\n}\n" ;
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
    #[derive(Clone, Debug)]
    pub enum ItemState {
        CREATED,
        DONE,
        ARCHIVED,
        DELETED,
        Other(String),
    }
    impl ::serde::Serialize for ItemState {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                ItemState::CREATED => "CREATED",
                ItemState::DONE => "DONE",
                ItemState::ARCHIVED => "ARCHIVED",
                ItemState::DELETED => "DELETED",
                ItemState::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemState {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "CREATED" => Ok(ItemState::CREATED),
                "DONE" => Ok(ItemState::DONE),
                "ARCHIVED" => Ok(ItemState::ARCHIVED),
                "DELETED" => Ok(ItemState::DELETED),
                _ => Ok(ItemState::Other(s)),
            }
        }
    }
    #[derive(Serialize, Clone, Debug)]
    pub struct Variables;
    #[derive(Deserialize, Clone, Debug)]
    pub struct ResponseData {
        #[serde(rename = "getProjects")]
        pub get_projects: Vec<GetProjectsListViewGetProjects>,
    }
    #[derive(Deserialize, Clone, Debug)]
    pub struct GetProjectsListViewGetProjects {
        pub id: UUID,
        pub name: String,
        pub items: Vec<GetProjectsListViewGetProjectsItems>,
    }
    #[derive(Deserialize, Clone, Debug)]
    pub struct GetProjectsListViewGetProjectsItems {
        pub id: UUID,
        pub title: String,
        pub description: Option<String>,
        pub state: ItemState,
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
