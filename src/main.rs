use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, 
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);
    let (tx, _rx) = broadcast::channel(10);
    // Now that there are .send and .rcv methods used. The compiler can infer the types
    // -> The Turbofish operator -> ::<T>()
    // https://matematikaadit.github.io/posts/rust-turbofish.html


    loop { // This helps supporting multiple clients, but the socket still works one client at a time.
        // There is a block happening at task level.
        // Sleeps one task until IO resource is ready.
        let (mut socket, addr ) = listener.accept().await.unwrap();
        
        // Move tx properly into the loop
        let tx = tx.clone();
        // rx cant be clone, instead needs to be created from tx.
        let mut rx = tx.subscribe();

        tokio::spawn(async move { // Solves the task blocking
            // Move all client handling into its own idependent task.
            // async move -> async block -> the whole block is it's own Future
            let (reader, mut writer) = socket.split(); // In order to solve ownership issue
            // -> when writing back to the socket (end of loop)
            
            let mut reader = BufReader::new(reader); // Wraps any kind of reader and keeps it's own reader 
            // providing Tokio's own functionality
            // socket cannot be moved itself because there is only one in the loop
            let mut line = String::new();
            loop {
                tokio::select! { // Like golang select
                    // the await statements happen implicitly
                    result = reader.read_line(&mut line) => {    
                        if result.unwrap() == 0 {
                            // The reader has reach the end of file. -> No more data left to read
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();  // Clear the input buffer content
                    }
                    result = rx.recv() => {
                        let (msg, sender_addr) = result.unwrap();
                        if addr != sender_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();  
                        }
                    }
                }
                // -> as bytes -> give the underlying bytes from the string.
            }
        });
    }
}
