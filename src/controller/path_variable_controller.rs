use axum::extract::Path;

pub async fn path_variables(Path((_, subid)): Path<(i32, i32)>) -> String {
    dbg!(&subid);
    subid.to_string()
}
