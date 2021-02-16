use bollard::container::StartContainerOptions;
use bollard::container::{Config, CreateContainerOptions};
use bollard::image::BuildImageOptions;
use bollard::models::BuildInfo;
use bollard::models::HostConfig;
use bollard::Docker;
use futures_util::stream::StreamExt;
use futures_util::stream::TryStreamExt;
use std::default::Default;
use std::env;
use std::fs;
use tokio;
extern crate tar;

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = env::args().collect();

    let dockerfile = &args[1];
    let imagename = args[2].clone();
    let containername = args[3].clone();

    let content = fs::read_to_string(dockerfile).unwrap();

    let mut header = tar::Header::new_gnu();
    header.set_path("Dockerfile").unwrap();
    header.set_size(content.len() as u64);
    header.set_mode(0o755);
    header.set_cksum();
    let mut tar = tar::Builder::new(Vec::new());
    tar.append(&header, content.as_bytes()).unwrap();
    let archive = tar.into_inner().unwrap();

    #[cfg(unix)]
    let docker = Docker::connect_with_unix_defaults().unwrap();

    async move {
        let options = BuildImageOptions {
            dockerfile: "Dockerfile",
            t: imagename.as_ref(),
            rm: true,
            ..Default::default()
        };

        let res = docker
            .build_image(options, None, Some(archive.into()))
            .map(|v| {
                println!("{:?}", v);
                v
            })
            .map_err(|e| {
                println!("{:?}", e);
                e
            })
            .collect::<Vec<Result<BuildInfo, bollard::errors::Error>>>()
            .await;
        println!("{:?}", res);

        let options = Some(CreateContainerOptions {
            name: containername.clone(),
        });
        let config = Config {
            image: Some(imagename),
            host_config: Some(HostConfig {
                binds: Some(vec![String::from("/tmp:/mnt")]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let res = docker.create_container(options, config).await.unwrap();
        println!("{:?}", res);
        let res = docker
            .start_container(
                containername.as_ref(),
                None::<StartContainerOptions<String>>,
            )
            .await;
        println!("{:?}", res);
    }
    .await;
}
