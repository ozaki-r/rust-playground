use tokio;
use bollard::Docker;
use bollard::container::ListContainersOptions;
use virt::connect::Connect;
use std::default::Default;

#[tokio::main]
pub async fn main() {
    #[cfg(unix)]
    let docker = Docker::connect_with_unix_defaults().unwrap();

    async move {
        let version = docker.version().await.unwrap();
        println!("{:?}", version);
        let containers = docker
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            })
            ).await.unwrap();
        println!("{:?}", containers);
    }.await;

    if let Ok(mut conn) = Connect::open("qemu:///system") {
        let domains = conn.list_domains().unwrap_or(vec![]);
        println!("{:?}", domains);
        conn.close().unwrap();
    }
}
