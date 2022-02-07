use tokio::{io::{AsyncReadExt, AsyncWriteExt} ,net::TcpListener};

#[tokio::main]
async fn main() {
    // listener which listens for any upcoming connections
    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    // variable containing the socket and address which accepts the incoming connection
    let (mut socket, _addr) = listener.accept().await.unwrap();

    // running an infinite loop enabling to read and write more than 1 message
    loop {
        // buffer to read data from the network stream
        let mut buffer = [0u8; 1024];
    
        /* read and store the data that the user provides in the network stream, eg: user types "hello", the data is read and stored */
        let bytes_read = socket.read(&mut buffer).await.unwrap();
    
        /* writing the data read back to the user. the data read and stored before is sent back to the user which in this case is "hello" */ 
        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}
