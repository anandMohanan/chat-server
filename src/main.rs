use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

// Main function is not allowed to use async , so we use a thing from tokio which lets us to use async in main
#[tokio::main]
async fn main() {
    // create a listener (TCP listener) which listens to the localhost , hence its an async function , we should use await
    let listener = TcpListener::bind("localhost:8080").await;

    //create a transmitter and reciever to create connection between two or more channels
    let (transmitter, _reciever) = broadcast::channel(10);
    // println!("{:#?}", listener);

    // listener error checking
    let listener = match listener {
        Ok(done) => done,
        Err(err) => {
            panic!("{}", err)
        }
    };

    // println!("{:#?}", listener);

    //loop because we dont want our chat server to end with only one message
    loop {
        //create a socket and the host addr by accepting the listener
        let (mut socket, addr) = listener.accept().await.unwrap();

        //to clear an errorrrr idk why :(
        let transmitter = transmitter.clone();
        let mut reciever = transmitter.subscribe();

        // [tokio spawn] because we want both of read and write to run concurrently
        tokio::spawn(async move {
            //getting read and write from the socket
            let (read, mut write) = socket.split();

            // New buffer reader
            let mut reader = BufReader::new(read);

            // create a string to store the values
            let mut line = String::new();
            loop {
                tokio::select! {
                    //read the line using the reader and store it in the line variable
                                    result = reader.read_line(&mut line) => {

                                        //if no value, then break
                                        if result.unwrap() == 0{
                                            break;
                                        }

                                        //transmitter send the line or msg with the address
                                        let _tx_send = transmitter.send((line.clone(),addr));

                                        //transmitter error check
                                        let _tx_send = match _tx_send {
                                            Ok(yes) => yes,
                                            Err(err) => panic!("{:#?}", err),
                                        };

                                        //clear the line or itll save the previous msgs in the buffer and send everything in the buffer
                                        line.clear();
                                    }

                                    //reciever recieves the message
                result = reciever.recv() => {

                    // get the message and the adress
                    let (msg,other_addr) = result.unwrap();

                    // if its the same address,dont send the message,if its a different adress send the msg
                    if addr != other_addr {
                        // write the messages
                        write.write_all(msg.as_bytes()).await.unwrap();
                    }


                }
                                }
            }
        });
    }
}
