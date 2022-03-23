use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, 
    net::TcpListener
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    // .await -> rust keyword. suspend the excecution until Future resolves
    let (mut socket, _addr ) = listener.accept().await.unwrap();
    // _addr -> has the _ prefix -> unused

    loop {
        let mut buffer = [0u8; 1024]; // 1kb (1024 bites)
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        // .read is a treat -> all items implementing async read -> imp wholeread
        // Need to use the asyncReadExt trait
        // -> requires buffer to be mutable
        socket.write_all(&buffer[..bytes_read]).await.unwrap();  
        // Write every single byte from the input buffer into the output buffer 
        // -> But tokio is handling some of the complexity
        // -> with AsyncWriteExt
    }
}
