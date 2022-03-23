use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, 
    net::TcpListener
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop { // This helps supporting multiple clients, but the socket still works one client at a time.
        // There is a block happening at task level.
        // Sleeps one task until IO resource is ready.
        let (mut socket, _addr ) = listener.accept().await.unwrap();
        
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
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                // Need to use the AsyncBufReadExt extention trait
                if bytes_read == 0 {
                    // The reader has reach the end of file. -> No more data left to read
                    break;
                }
                writer.write_all(line.as_bytes()).await.unwrap();  
                // -> as bytes -> give the underlying bytes from the string.
                line.clear();  // Clear the input buffer content
            }
        });
    }
}
