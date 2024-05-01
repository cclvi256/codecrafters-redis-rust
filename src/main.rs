use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:6379").await.unwrap();
    
    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((mut stream, _)) => {
                println!("accepted new connection");
                
                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    loop {
                        let read_count = stream.read(&mut buf).await.unwrap();
                        if read_count == 0 {
                            break;
                        }
                        
                        stream.write(b"+PONG\r\n").await.unwrap();
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
