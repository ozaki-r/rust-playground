use tokio;
use bollard::Docker;

#[tokio::main]
pub async fn main() {
    #[cfg(unix)]
    let docker = Docker::connect_with_unix_defaults().unwrap();

    async move {
        let version = docker.version().await.unwrap();
        println!("{:?}", version);
    }.await;
}
