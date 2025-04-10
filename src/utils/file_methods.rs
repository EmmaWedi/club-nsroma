pub async fn save_file(
    sub_path: &str,
    file_name: &str,
    extension: &str,
    data: &[u8],
) -> Result<(), std::io::Error> {
    let dir_path = format!("uploads/{}", sub_path);
    let file_path = format!("uploads/{}/{}.{}", sub_path, file_name, extension);

    tokio::fs::create_dir_all(dir_path).await?;

    tokio::fs::write(file_path, data).await
}

pub async fn read_file(
    sub_path: &str,
    file_name: &str,
    extension: &str,
) -> Result<Vec<u8>, std::io::Error> {
    let file_path = format!("uploads/{}/{}.{}", sub_path, file_name, extension);
    tokio::fs::read(file_path).await
}

pub async fn file_exists(sub_path: &str, file_name: &str, extension: &str) -> bool {
    let file_path = format!("uploads/{}/{}.{}", sub_path, file_name, extension);
    tokio::fs::metadata(file_path).await.is_ok()
}
