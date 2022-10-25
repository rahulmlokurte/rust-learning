use std::io;
use std::time;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {

}

fn handle_sender(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0;512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;

        println!("from the sender:{}",String::from_utf8_lossy(&buf));
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
