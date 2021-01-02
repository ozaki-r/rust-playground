use tokio;
use bollard::Docker;
use virt::connect::Connect;

#[tokio::main]
pub async fn main() {
    #[cfg(unix)]
    let docker = Docker::connect_with_unix_defaults().unwrap();

    async move {
        let version = docker.version().await.unwrap();
        println!("{:?}", version);
    }.await;

    if let Ok(mut conn) = Connect::open("qemu:///system") {
        let domains = conn.list_domains().unwrap_or(vec![]);
        println!("{:?}", domains);
        conn.close().unwrap();
    }
}
