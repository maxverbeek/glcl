use anyhow::{Ok, Result};
use sqlx::{Connection, SqliteConnection};
use tracing::debug;

#[derive(sqlx::FromRow, Debug)]
pub struct Project {
    pub id: u32,
    pub description: Option<String>,
    pub name: String,
    pub name_with_namespace: String,
    pub path: String,
    pub path_with_namespace: String,
    pub created_at: String,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub web_url: String,
    pub avatar_url: Option<String>,
    pub last_activity_at: String,
    pub parent_avatar_url: Option<String>,
}

pub struct SQLiteDatabase {
    connection: SqliteConnection,
}

impl SQLiteDatabase {
    pub async fn try_new() -> Result<Self> {
        let connection = SqliteConnection::connect("sqlite://projects.db?mode=rwc").await?;
        // also create tables
        let mut db = Self { connection };

        db.create_projects_table().await?;

        return Ok(db);
    }

    async fn create_projects_table(&mut self) -> Result<()> {
        debug!(name: "create_projects_table", message = "creating projects table");
        let query = r#"CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            description TEXT,
            name TEXT NOT NULL,
            name_with_namespace TEXT NOT NULL,
            path TEXT NOT NULL,
            path_with_namespace TEXT NOT NULL,
            created_at TEXT NOT NULL,
            ssh_url_to_repo TEXT NOT NULL,
            http_url_to_repo TEXT NOT NULL,
            web_url TEXT NOT NULL,
            avatar_url TEXT,
            last_activity_at TEXT,
            parent_avatar_url TEXT
        )"#;

        sqlx::query(query).execute(&mut self.connection).await?;

        Ok(())
    }

    pub async fn insert_projects(&mut self, projects: &[Project]) -> Result<()> {
        for project in projects {
            sqlx::query(
                r#"
                INSERT OR REPLACE INTO projects (
                    id,
                    description,
                    name,
                    name_with_namespace,
                    path, path_with_namespace,
                    created_at,
                    ssh_url_to_repo,
                    http_url_to_repo,
                    web_url,
                    avatar_url,
                    last_activity_at,
                    parent_avatar_url
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(project.id)
            .bind(&project.description)
            .bind(&project.name)
            .bind(&project.name_with_namespace)
            .bind(&project.path)
            .bind(&project.path_with_namespace)
            .bind(&project.created_at)
            .bind(&project.ssh_url_to_repo)
            .bind(&project.http_url_to_repo)
            .bind(&project.web_url)
            .bind(&project.avatar_url)
            .bind(&project.last_activity_at)
            .bind(&project.parent_avatar_url)
            .execute(&mut self.connection)
            .await?;
        }

        Ok(())
    }

    pub async fn get_projects(&mut self) -> Result<Vec<Project>> {
        let rows: Vec<Project> =
            sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY id ASC")
                .fetch_all(&mut self.connection)
                .await?;

        Ok(rows)
    }

    pub async fn count_projects(&mut self) -> Result<u64> {
        let count: u64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects")
            .fetch_one(&mut self.connection)
            .await?;

        Ok(count)
    }
}
