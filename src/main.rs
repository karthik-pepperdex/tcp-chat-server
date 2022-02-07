use tokio::{io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, net::TcpListener, sync::broadcast};

#[tokio::main]
async fn main() {
    // listener which listens for any upcoming connections
    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    // feature to broadcast a message sent by a user to all the users
    let (tx, _rx) = broadcast::channel(10);

    // infinite loop which will accept any number of incoming connections
    loop {
        // variable containing the socket and address which accepts the incoming connection
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();

        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
        
            // buffer that reads and stores data from the network stream
            let mut reader = BufReader::new(reader);
            
            // the data that the user provides
            let mut line = String::new();
            // running an infinite loop enabling to read and write more than 1 message
            loop {
                // tokio select to see which await statement responds with data first and act accordingly
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear()
                    } result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        // if the address is not the same as the other addresses, send back the message to the other users, otherwise do not
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    
    }

}
