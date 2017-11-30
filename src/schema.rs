infer_schema!("dotenv:DATABASE_URL");

joinable!(recipes -> users (user_id));
