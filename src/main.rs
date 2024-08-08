use std::{collections::HashMap, env, fs, path::PathBuf};

use anyhow::Result;
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()> {
    let gl = Gitlab::try_new()?;

    dbg!(gl.get_projects().await);

    Ok(())
}

#[derive(Debug)]
struct GetProjectsResponse(Vec<GitlabProject>, Option<PaginationInfo>);

#[derive(Debug, Deserialize)]
struct GitlabProject {
    id: u32,
    description: Option<String>,
    name: String,
    name_with_namespace: String,
    path: String,
    path_with_namespace: String,
    created_at: String,
    default_branch: Option<String>,
    tag_list: Vec<String>,
    topics: Vec<String>,
    ssh_url_to_repo: String,
    http_url_to_repo: String,
    web_url: String,
    avatar_url: Option<String>,
    star_count: u32,
    last_activity_at: String,
    namespace: GitlabProjectNamespace,
}

#[derive(Debug, Deserialize)]
struct GitlabProjectNamespace {
    id: u32,
    name: String,
    path: String,
    kind: String,
    full_path: String,
    parent_id: Option<u32>,
    avatar_url: Option<String>,
    web_url: String,
}

#[derive(Debug)]
struct Gitlab {
    pat: String,
    client: Client,
}

#[derive(Clone, Debug)]
struct PaginationInfo {
    id_after: usize,
}

impl Gitlab {
    fn try_new() -> Result<Self> {
        let filepath = env::var("HOME").map(PathBuf::from)?.join(".gitlab_pat");

        let token = fs::read_to_string(filepath)?;
        let token = token.trim();

        Ok(Gitlab {
            pat: token.to_owned(),
            client: ClientBuilder::new().build()?,
        })
    }

    async fn get_projects(&self) -> Result<Vec<GitlabProject>> {
        let mut projects: Vec<GitlabProject> = vec![];
        let mut pagination = None;
        loop {
            let GetProjectsResponse(myprojects, next_cursor) =
                self.get_projects_page(pagination).await?;

            projects.extend(myprojects);

            match next_cursor {
                None => break,
                next_pagination => pagination = next_pagination,
            }
        }

        Ok(projects)
    }

    async fn get_projects_page(
        &self,
        pagination_info: Option<PaginationInfo>,
    ) -> Result<GetProjectsResponse> {
        let url = "https://gitlab.com/api/v4/projects";

        let mut query_params = HashMap::new();
        query_params.insert("pagination", "keyset".to_owned());
        query_params.insert("order_by", "id".to_owned());
        query_params.insert("sort", "asc".to_owned());
        query_params.insert("per_page", "100".to_owned());
        query_params.insert("membership", "true".to_owned());

        if let Some(pagination) = pagination_info {
            query_params.insert("id_after", pagination.id_after.to_string());
        }

        let response = self
            .client
            .get(url)
            .query(&query_params)
            .header("PRIVATE-TOKEN", self.pat.clone())
            .send()
            .await?
            .error_for_status()?;

        let next_page_info = get_next_page_info(response.headers());
        let projects = response.json::<Vec<GitlabProject>>().await?;

        Ok(GetProjectsResponse(projects, next_page_info))
    }
}

fn get_next_page_info(headers: &HeaderMap) -> Option<PaginationInfo> {
    let link_header = headers.get("link")?;
    let link = parse_link_header::parse_with_rel(link_header.to_str().ok()?).ok()?;

    let next = link.get("next")?;
    let id_after = next.queries.get("id_after")?;

    Some(PaginationInfo {
        id_after: str::parse::<usize>(id_after).ok()?,
    })
}
