use std::error::Error;

use tokio::io::{self, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = "publicdatafeeds.networkrail.co.uk:61618";

    let stream = tokio::net::TcpStream::connect(socket).await?;

    let (mut rd, mut wr) = io::split(stream);

    let connect = tokio::fs::read("./connect.txt").await?;
    let sub = tokio::fs::read("./subscribe.txt").await?;

    // Write data in the background
    tokio::spawn(async move {
        wr.write_all(&connect).await?;

        wr.write_all(&sub).await?;

        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        let string = String::from_utf8(buf.clone()).unwrap();

        println!("GOT {:?}", string);
    }

    Ok(())
}
