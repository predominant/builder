use super::{db_id_format, db_optional_id_format};
use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use models::package::PackageVisibility;
use protocol::originsrv;
use schema::jobs::jobs;

#[derive(Debug, Serialize, Deserialize, QueryableByName, Queryable)]
#[table_name = "jobs"]
pub struct Job {
    #[serde(with = "db_id_format")]
    pub id: i64,
    #[serde(with = "db_id_format")]
    pub owner_id: i64,
    pub job_state: String,
    #[serde(with = "db_id_format")]
    pub project_id: i64,
    pub project_name: String,
    #[serde(with = "db_id_format")]
    pub project_owner_id: i64,
    pub project_plan_path: String,
    pub vcs: String,
    pub vcs_arguments: String,
    pub net_error_code: i32,
    pub net_error_msg: String,
    pub scheduler_sync: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub build_started_at: Option<NaiveDateTime>,
    pub build_finished_at: Option<NaiveDateTime>,
    pub package_ident: String,
    pub archived: bool,
    pub channel: String,
    pub sync_count: i32,
    pub worker: String,
}
