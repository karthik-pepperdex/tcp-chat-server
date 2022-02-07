use tokio::{io::{AsyncWriteExt, BufReader, AsyncBufReadExt} ,net::TcpListener};

#[tokio::main]
async fn main() {
    // listener which listens for any upcoming connections
    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    // variable containing the socket and address which accepts the incoming connection
    let (mut socket, _addr) = listener.accept().await.unwrap();

    let (reader, mut writer) = socket.split();

    // buffer that reads and stores data from the network stream
    let mut reader = BufReader::new(reader);
    
    // the data that the user provides
    let mut line = String::new();
    // running an infinite loop enabling to read and write more than 1 message
    loop {
        /* the bytes that the server read from the data that the user provided */
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }
    
        /* writing the data read back to the user. the data read and stored before is sent back to the user which in this case is "hello" */ 
        writer.write_all(line.as_bytes()).await.unwrap();

        /* clears the line after its been printed back by the server to prevent the present line being printed again when the next message 
        needs to be printed */
        line.clear();
    }
}
