table! {
    use diesel::sql_types::{Bool, Integer, BigInt, Text, Nullable, Timestamptz};

    jobs (id) {
        id -> BigInt,
        owner_id -> BigInt,
        job_state -> Text,
        project_id -> BigInt,
        project_name -> Text,
        project_owner_id -> BigInt,
        project_plan_path -> Text,
        vcs -> Text,
        vcs_arguments-> Text,
        net_error_code -> Integer,
        net_error_msg -> Text,
        scheduler_sync -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        build_started_at -> Nullable<Timestamptz>,
        build_finished_at -> Nullable<Timestamptz>,
        package_ident -> Text,
        archived -> Bool,
        channel -> Text,
        sync_count -> Integer,
        worker -> Text,
    }
}

table! {
    use diesel::sql_types::{BigInt, Text, Nullable, Timestamptz};

    groups (id) {
        id -> BigInt,
        group_state -> Text,
        project_name -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    use diesel::sql_types::{BigInt, Text, Nullable, Timestamptz};

    group_projects (id) {
        id -> BigInt,
        owner_id -> BigInt,
        project_name -> Text,
        project_ident -> Text,
        project_state -> Text,
        job_id -> BigInt,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    use diesel::sql_types::{BigInt, Bool, Text, Nullable, Timestamptz};

    busy_workers(ident, job_id) {
        ident -> Text,
        job_id -> BigInt,
        quarantined -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}
