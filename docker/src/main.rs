use bollard::container::ListContainersOptions;
use bollard::container::{Config, CreateContainerOptions};
use bollard::container::StartContainerOptions;
use bollard::Docker;
use std::default::Default;
use tokio;
use std::env;

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = env::args().collect();

    let image = &args[1];
    let name = &args[2];

    let docker = Docker::connect_with_unix_defaults().unwrap();

    let version = docker.version().await.unwrap();
    println!("{:?}", version);
    let containers = docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();
    println!("{:?}", containers);

    let options = Some(CreateContainerOptions {
        name,
    });
    let config = Config {
        image: Some(image.to_owned()),
        ..Default::default()
    };
    let res = docker.create_container(options, config).await.unwrap();
    println!("{:?}", res);
    let res = docker.start_container(name, None::<StartContainerOptions<String>>).await;
    println!("{:?}", res);
}
