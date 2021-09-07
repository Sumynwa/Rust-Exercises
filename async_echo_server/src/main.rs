use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        println!("Socket: {:?} addr: {:?}", socket, addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop {
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => {break;},
                    Ok(n) => n,
                    Err(e) => {
                                  println!("Error unable to read from socket, err = {:?}", e);
                                  return;
                              },
                 };

             if let Err(e) = socket.write_all(&buffer[0..n]).await {
                 println!("Error unable to write to socket, err = {:?}", e);
                 return;
              }
             }
       });
    }
}
