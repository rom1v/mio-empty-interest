extern crate mio;
use std::io::{self, ErrorKind, Read, Write};
use std::net::{self, Ipv4Addr, SocketAddr};
use std::thread;
use std::time::{Duration, Instant};
use mio::{Events, Poll, PollOpt, Ready, Token};

fn client() {
    let addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 1234);
    let mut stream = mio::tcp::TcpStream::connect(&addr).unwrap();

    let poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(16);
    let mut buf = [0; 16];

    let timeout = Some(Duration::from_millis(5500));

    println!("interest: {{readable}}");
    poll.register(&stream, Token(0), Ready::readable(), PollOpt::level()).unwrap();
    read_once(&poll, &mut stream, &mut events, &mut buf, timeout).unwrap();
    read_once(&poll, &mut stream, &mut events, &mut buf, timeout).unwrap();

    println!("interest: {{}}");
    poll.reregister(&stream, Token(0), Ready::empty(), PollOpt::level()).unwrap();
    read_once(&poll, &mut stream, &mut events, &mut buf, timeout).unwrap();

    println!("interest: {{readable}}");
    poll.reregister(&stream, Token(0), Ready::readable(), PollOpt::level()).unwrap();
    read_once(&poll, &mut stream, &mut events, &mut buf, timeout).unwrap();
    read_once(&poll, &mut stream, &mut events, &mut buf, timeout).unwrap();
    read_once(&poll, &mut stream, &mut events, &mut buf, timeout).unwrap();
}

fn read_once(poll: &Poll, stream: &mut mio::tcp::TcpStream, events: &mut Events, buf: &mut [u8], timeout: Option<Duration>) -> io::Result<()> {
    let start = Instant::now();
    loop {
        let elapsed = start.elapsed();
        let remaining = if let Some(timeout) = timeout {
            if elapsed >= timeout {
                return Ok(());
            }
            Some(timeout - elapsed)
        } else {
            None
        };
        poll.poll(events, remaining).unwrap();
        for event in &*events {
            if event.readiness().is_readable() {
                match stream.read(buf) {
                    Ok(0) => {
                        println!("EOF");
                        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"));
                    }
                    Ok(len) => {
                        let content = String::from_utf8_lossy(&buf[..len]);
                        println!("      --> {}", content);
                        return Ok(());
                    }
                    Err(err) => {
                        if err.kind() == ErrorKind::WouldBlock {
                            // spurious event, ignoring
                        } else {
                            println!("CLIENT (receiver): error [{:?}]: {}", err.kind(), err);
                            return Err(err);
                        }
                    }
                }
            }
        }
    }
}

fn server() {
    let addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 1234);
    let server = net::TcpListener::bind(&addr).unwrap();
    let (mut stream, _) = server.accept().unwrap();
    for i in 0..9 {
        thread::sleep(Duration::from_secs(1));
        println!("{} -->", i);
        stream.write(format!("{}", i).as_bytes()).unwrap();
    }
}

fn main() {
    let thread = thread::spawn(server);
    thread::sleep(Duration::from_secs(1));
    client();
    thread.join().unwrap();
}
