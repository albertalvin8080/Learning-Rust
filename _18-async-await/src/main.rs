// NOTE: tokio said to import from std instead.
// use tokio::net::{TcpListener, TcpStream};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

async fn handle_connection(socket: TcpStream) -> anyhow::Result<()> {
    /* 
    TcpStream implements the Clone trait by internally using reference-counted 
    shared ownership of the underlying file descriptor (or socket handle). This 
    shared ownership allows you to create multiple TcpStream instances that point 
    to the same underlying socket.
    */
    let mut reader: BufReader<TcpStream> = BufReader::new(socket.try_clone()?);
    let mut writer: BufWriter<TcpStream> = BufWriter::new(socket);
    let mut message = String::new();

    reader.read_line(&mut message)?;
    println!("{}", message);

    writer.write_all(message.as_bytes())?;
    writer.flush()?;

    // sleep(Duration::from_millis(10_000));
    
    Ok(())
    // At the end of the scope, all TcpStream instances are dropped, and when
    // the reference counter reaches 0, the connection is lost.
}

// TOML -> {..., features = ["rt-multi-thread", "macros"] ...}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    /*
    1. Enable telnet on windows.
    2. telnet 127.0.0.1 6189
    3. send messages (you may not be able to see your input, but you ARE typing).
    */
    let listener = TcpListener::bind("127.0.0.1:6189").unwrap();

    loop {
        let (socket, _addr) = listener.accept().unwrap();
        handle_connection(socket).await?;
    }
}
