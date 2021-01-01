use tokio::runtime::Runtime;
use bollard::Docker;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    let docker = Docker::connect_with_unix_defaults().unwrap();

    async move {
        let version = docker.version().await.unwrap();
        println!("{:?}", version);
    }.await;
    Ok(())
}

fn main() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(run()).unwrap();
}
