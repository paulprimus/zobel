
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use tokio::{io::{AsyncReadExt, AsyncWriteExt},{fs::File, net::TcpStream}};

#[cfg(target_family="windows")]
use std::os::windows::fs::MetadataExt;

const DEFAULT_CHUNK_SIZE: usize = 4096;

pub(crate) async fn ping() -> Result<()> {
    tracing::info!("Requesting Ping!");
    let mut stream = create_stream().await?;
    stream.write_all(b"zPING\0").await?;
    // let capacity = b"PONG\n".len();
    let mut response = Vec::with_capacity(5);
    stream.read_to_end(&mut response).await?;
    tracing::info!("Response: {:?}", String::from_utf8(response)?);
    Ok(())
}

pub(crate) async fn stats() -> Result<()> {
    tracing::info!("Requesting STATS!");
    let mut stream = create_stream().await?;
    stream.write_all(b"zSTATS\0").await?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;
    tracing::info!("Response: {:?}", String::from_utf8(response)?);
    Ok(())
}

pub(crate) async fn instream(filename: String) -> Result<()> {
    tracing::info!("Requesting INSTREAM!");

    let mut stream = create_stream().await?;
    stream.write_all(b"zINSTREAM\0").await?;

    let mut buffer = [0; DEFAULT_CHUNK_SIZE];
    let mut file = File::open(filename).await?;
    let metadata = file.metadata().await?;
    tracing::debug!("Filesize: {}", metadata.file_size());
    loop {

        let length = file.read(&mut buffer[..]).await?;
        // tracing::debug!("length: {}", length);
        if length != 0 {
            stream.write_all(&(length as u32).to_be_bytes()).await?;
            stream.write_all(&buffer[..length]).await?;
        } else {
            stream.write_all(&[0; 4]).await?;
            break;
        }
    }

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;
    tracing::info!("Response: {:?}", String::from_utf8(response)?);
    Ok(())
}

async fn create_stream() -> Result<TcpStream> {
    let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 34, 38));
    let socket_addr = SocketAddr::new(ip_addr, 3310u16);
    let socket = tokio::net::TcpSocket::new_v4()?;
    let stream = socket.connect(socket_addr).await?;
    Ok(stream)
}
