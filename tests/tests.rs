use core::time;
use std::{
    io::{BufRead, BufReader, Write},
    net::{IpAddr, Shutdown, SocketAddr, TcpStream},
    process::{Child, Command},
    str::FromStr,
    thread,
};

use rand::Rng;

const BINARY_PATH: &str = env!("CARGO_BIN_EXE_hash-delivery-service");

enum IpVersion {
    V4,
    V6,
}

struct ServerWrapper {
    proc: Option<Child>,
    addr: SocketAddr,
}

impl ServerWrapper {
    fn start(ip_version: IpVersion) -> Self {
        let mut rng = rand::thread_rng();
        let port = rng.gen_range(40000..49151);
        let ip = match ip_version {
            IpVersion::V4 => IpAddr::from_str("127.0.0.1").unwrap(),
            IpVersion::V6 => IpAddr::from_str("::1").unwrap(),
        };

        eprintln!("binary path {}", BINARY_PATH);
        let proc = Command::new(BINARY_PATH)
            .arg("--ip")
            .arg(ip.to_string())
            .arg("--port")
            .arg(port.to_string())
            .spawn()
            .unwrap();
        thread::sleep(time::Duration::from_millis(1000));
        Self {
            proc: Some(proc),
            addr: SocketAddr::new(ip, port),
        }
    }

    fn stop(&mut self) -> std::io::Result<()> {
        self.proc.take().map_or(Ok(()), |mut proc| proc.kill())
    }
}

impl Drop for ServerWrapper {
    fn drop(&mut self) {
        let _ = self.stop().unwrap();
    }
}

#[derive(Debug)]
struct Client {
    conn: TcpStream,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            conn: self.conn.try_clone().unwrap(),
        }
    }
}

impl Client {
    fn connect(server_addr: SocketAddr) -> std::io::Result<Self> {
        let conn = TcpStream::connect(server_addr)?;
        Ok(Self { conn })
    }

    fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.conn.write_all(data)
    }

    fn read_expect(&mut self, expected: String) -> std::io::Result<()> {
        let mut buf = vec![0; expected.len() + 1];
        let mut reader = BufReader::new(&mut self.conn);
        reader.read_until(b'}', &mut buf)?;
        let mut incoming = String::from_utf8(buf).unwrap();
        incoming = delete_char(' ', incoming);
        assert_eq!(expected, delete_char('\0', incoming));
        Ok(())
    }

    fn shutdown(&mut self, how: Shutdown) {
        let _ = self.conn.shutdown(how);
    }
}

fn make_store_request(key: &str, hash: &str) -> String {
    format!(
        r#"{{
        "request_type": "store",
        "key": "{key}",
        "hash": "{hash}"}}
    "#
    )
}

fn make_load_request(key: &str) -> String {
    format!(
        r#"{{
        "request_type": "load",
        "key": "{key}"}}
    "#
    )
}

fn make_greeting() -> String {
    delete_char(' ', format!(r#"{{"student_name":"Gordei Skorobogatov"}}"#))
}

fn make_success_store() -> String {
    delete_char(' ', format!(r#"{{"response_status": "success"}}"#))
}

fn make_success_load(key: &str, hash: &str) -> String {
    delete_char(
        ' ',
        format!(
            r#"{{"response_status": "success", "requested_key": "{key}", "requested_hash": "{hash}"}}"#
        ),
    )
}

fn make_no_key() -> String {
    delete_char(' ', format!(r#"{{"response_status": "key not found"}}"#))
}

fn make_error() -> String {
    delete_char(
        ' ',
        format!(r#"{{"response_status": "error"}}"#).replace("-", " "),
    )
}

fn delete_char(ch: char, mut str: String) -> String {
    str.retain(|i| i != ch);
    str
}

#[test]
fn test_single_login() {
    let server = ServerWrapper::start(IpVersion::V4);
    let mut client = Client::connect(server.addr).unwrap();
    client.read_expect(make_greeting()).unwrap();

    client.shutdown(Shutdown::Both);
    drop(server);
}

#[test]
fn test_store_request() {
    let server = ServerWrapper::start(IpVersion::V4);
    let mut client = Client::connect(server.addr).unwrap();
    client.read_expect(make_greeting()).unwrap();

    client
        .write(make_store_request("aboba", "baboba").as_bytes())
        .unwrap();
    client.read_expect(make_success_store()).unwrap();

    client.shutdown(Shutdown::Both);
    drop(server);
}

#[test]
fn test_load_request() {
    let server = ServerWrapper::start(IpVersion::V4);
    let mut client = Client::connect(server.addr).unwrap();
    client.read_expect(make_greeting()).unwrap();

    client
        .write(make_store_request("aboba", "baboba").as_bytes())
        .unwrap();
    client.read_expect(make_success_store()).unwrap();

    client.write(make_load_request("aboba").as_bytes()).unwrap();
    client
        .read_expect(make_success_load("aboba", "baboba"))
        .unwrap();

    client.shutdown(Shutdown::Both);
    drop(server);
}

#[test]
fn test_no_key_load_request() {
    let server = ServerWrapper::start(IpVersion::V4);
    let mut client = Client::connect(server.addr).unwrap();
    client.read_expect(make_greeting()).unwrap();

    client
        .write(make_store_request("aboba", "baboba").as_bytes())
        .unwrap();
    client.read_expect(make_success_store()).unwrap();

    client.write(make_load_request("chel").as_bytes()).unwrap();
    client.read_expect(make_no_key()).unwrap();

    client.shutdown(Shutdown::Both);
    drop(server);
}

#[test]
fn test_error() {
    let server = ServerWrapper::start(IpVersion::V4);
    let mut client = Client::connect(server.addr).unwrap();
    client.read_expect(make_greeting()).unwrap();

    client
        .write(delete_char(' ', format!(r#"{{"response_status": "nothing"}}"#)).as_bytes())
        .unwrap();
    client.read_expect(make_error()).unwrap();

    client.shutdown(Shutdown::Both);
    drop(server);
}
