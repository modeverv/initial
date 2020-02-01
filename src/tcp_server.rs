use std::io::{Read,Write};
use std::net::{TcpListener, TcpStream};
use std::{str,thread};

/**
 * ソケットアドレスで待受
 */
pub fn serve(address: &str) -> Result<(),failure::Error> {
    let listner = TcpListener::bind(address)?;
    /* [1] */
    loop {
        let (stream, _)  = listner.accept()?;
        // make thread
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}",error));
        });
    }
}
/**
 * そのままエコー処理
 */
fn handler(mut stream: TcpStream) -> Result<(),failure::Error> {
    debug!("Handling data from {}",stream.peer_addr()?);
    let mut buffer = [0u8;2048];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        println!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}