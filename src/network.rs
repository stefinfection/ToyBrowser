extern crate native_tls;

use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

/* Parses a provided url argument for the host, port, path, and fragment.
 * Prints the
 */
pub fn parse_url(url: &str, print_pieces: bool) -> (String, String, String, String) {
    // Check that URL starts w/ http
    assert!(url.starts_with("http://"));

    // Strip of http
    let stripped_url = &url[7..];
    println!("stripped url: {}", stripped_url);

    // Split the individual pieces of url out
    let mut host: String = "".to_string();
    let mut port: String = ":".to_string();
    let mut path: String = "/".to_string();
    let mut frag: String = "#".to_string();

    if stripped_url.contains("/") {
        let hostport_pathfrag: Vec<&str> = stripped_url.splitn(2, '/').collect();
        let hostport: &str = hostport_pathfrag[0];
         let mut pathfrag = "";
        match hostport_pathfrag.get(1) {
            Some(_) => { pathfrag = hostport_pathfrag[1]; },
            None => {}
        }

        // Parse host and port
        if hostport.contains(":") {
            let host_port: Vec<&str> = hostport.splitn(2,':').collect();
            host = host_port[0].to_string();
            port.push_str(&host_port[1]);
        } else {
            host = hostport.to_string();
            port = "".to_string();
        }

        // Parse path and fragment
        if pathfrag.contains("#") {
            let path_frag: Vec<&str> = pathfrag.splitn(2, "#").collect();
            path.push_str(&path_frag[0]);
            frag.push_str(&path_frag[1]);
        } else {
            path = pathfrag.to_string();
            frag = "".to_string();
        }
    } else {
        host = stripped_url.to_string();
    }

    // Print out the pieces
    if print_pieces {
        println!("host: {}", host);
        println!("port: {}", port);
        println!("path: {}", path);
        println!("frag: {}", frag);
    }

    return (host, port, path, frag);
}

/* Requests information from a server with the provided arguments.
 * Returns the header as a dictionary and body as a string.
 */
pub fn request(host: &str, port: &str, path: &str) -> () {
    // Empty checks
    if host == "" {
        panic!("Error: no host provided to request function");
    }
    let mut port: &str = port;
    if port == "" {
        port = ":8080";
    }

    // Open a socket
    let connector = TlsConnector::new().unwrap();
    let mut tcp_input: String = host.to_string();
    tcp_input.push_str(port);
    let stream = TcpStream::connect(tcp_input).unwrap();
    let mut stream = connector.connect(host, stream).unwrap();

    // Write to the stream
    let header:String = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", path, host);
    stream.write_all(header.as_bytes()).unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    // TODO: split the return stream into header and body

//    println!("{}", String::from_utf8_lossy(&res));
}

/* Prints the body from the provided html to the console. */
fn show(html: &str) -> () {
    if html == ""{
        panic!("String provided to show function is null");
    }

    let mut in_tag :bool = false;
    let mut tag_name:String = "".to_string();
    for c in html.chars() {
        if c == '<'{
            in_tag = true;
            tag_name = "".to_string();
        } else if c == '>'{
            in_tag = false;
        } else {
            if !in_tag && "body".to_string() == tag_name {
                print!("{}", c);
            } else if in_tag {
                tag_name.push(c);
            }
        }
    }
}

/* TESTS */
#[test]
pub fn test_url() {
    assert_eq!()
}