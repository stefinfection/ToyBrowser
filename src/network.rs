/* Controls connecting to an endpoint and parsing urls.
 *
 * Ref: https://github.com/sfackler/rust-native-tls
 */

extern crate native_tls;

use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::collections::HashMap;

/* Parses a provided url argument for the host, port, path, and fragment.
 * Prints the extracted pieces of the url, in the above order, if print_pieces is true.
 */
pub fn parse_url(url: &str, print_pieces: bool) -> (String, String, String, String) {
    // Check that URL starts w/ http
    assert!(url.starts_with("http://"));

    // Strip of http
    let stripped_url:&str = &url[7..];
    if print_pieces {
        println!("stripped url: {}", stripped_url);
    }

    // Split the individual pieces of url out
    let mut host: String = "".to_string();
    let mut port: String = ":".to_string();
    let mut path: String = "/".to_string();
    let mut frag: String = "#".to_string();

    if stripped_url.contains("/") {
        let hostport_pathfrag: Vec<&str> = stripped_url.splitn(2, '/').collect();
        let hostport: &str = hostport_pathfrag[0];
        let mut pathfrag: &str = "";
        match hostport_pathfrag.get(1) {
            Some(_) => { pathfrag = hostport_pathfrag[1]; }
            None => {}
        }

        // Parse host and port
        if hostport.contains(":") {
            let host_port: Vec<&str> = hostport.splitn(2, ':').collect();
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
            if pathfrag == "" {
                path = "".to_string();
            } else {
                path.push_str(&pathfrag);
            }
            frag = "".to_string();
        }
    } else {
        host = stripped_url.to_string();
        port = "".to_string();
        path = "".to_string();
        frag = "".to_string();
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
pub fn request(host: &str, port: &str, path: &str) -> (String, String, String, HashMap<String, String>, String) {
    // Empty check
    if host == "" {
        panic!("Error: no host provided to request function");
    }
    let host_port = format!("{}{}", host, port);

    // Open a socket
    let connector: TlsConnector = TlsConnector::new().unwrap();
    let mut tcp_input: String = host.to_string();
    tcp_input.push_str(":443");     // Standard TCP port
    let stream: TcpStream = TcpStream::connect(tcp_input).unwrap();
    let mut stream = connector.connect(&host_port, stream).unwrap();

    // Write to the stream
    let header: String = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", path, &host_port);
    stream.write_all(&header.as_bytes()).unwrap();

    // Read from stream
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();

    // Parse header
    let response: String = String::from_utf8_lossy(&res).to_string();
    let header_body: Vec<&str> = (&response).splitn(2, "\r\n\r\n").collect();
    let header: String = header_body[0].to_string();
    let header_lines:Vec<&str> = header.split("\r\n").collect();

    // Pull out first line
    let first_line:Vec<&str> = header_lines[0].splitn(3, " ").collect();
    let version: String = first_line[0].to_string();
    let status: String = first_line[1].to_string();
    let explanation: String = first_line[2].to_string();

    // Populate header dictionary
    let mut header_map:HashMap<String,String>= HashMap::new();
    for line in &header_lines[1..] {
        let key_val:Vec<&str> = line.splitn(2, " ").collect();
        header_map.insert(key_val[0].to_string(), key_val[1].to_string());
    }

    // Extract body
    let body: String = header_body[1].to_string();

    return (version, status, explanation, header_map, body);
}

/* Returns the inside of the <body> tag provided as the html argument.
 * Returns an empty string upon the argument not containing a body tag.
 */
pub fn show(html: &str) -> (String) {
    if html == "" {
        panic!("String provided to show function is null");
    }

    let mut body_text: String = "".to_string();

    let mut in_tag: bool = false;
    let mut tag_name: String = "".to_string();
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
            tag_name = "".to_string();
        } else if c == '>' {
            in_tag = false;
        } else {
            if !in_tag && "body".to_string() == tag_name {
                body_text.push(c);
            } else if in_tag {
                tag_name.push(c);
            }
        }
    }
    return body_text;
}


/* TESTS */
#[cfg(test)]
mod network_tests {
    use super::*;

    #[test]
    fn test_parse_host_only() {
        let actual = parse_url("http://example.org", false);
        let expected = ("example.org".to_string(),"".to_string(),"".to_string(),"".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_port() {
        let actual = parse_url("http://localhost:8080/", false);
        let expected = ("localhost".to_string(),":8080".to_string(),"".to_string(),"".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_path() {
        let actual = parse_url("http://example.org/index.html", false);
        let expected = ("example.org".to_string(),"".to_string(),"/index.html".to_string(),"".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_frag() {
        let actual = parse_url("http://example.org/index.html#head", false);
        let expected = ("example.org".to_string(),"".to_string(),"/index.html".to_string(),"#head".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn test_request_empty() {
        request("", "", "");
    }

    #[test]
    fn test_request_example() {
        let actual = request("example.org", "", "/index.html");
        let actual_version = actual.0;
        let actual_status = actual.1;
        let actual_explanation = actual.2;
        let actual_body = actual.4;

        assert_eq!(actual_version, "HTTP/1.0".to_string());
        assert_eq!(actual_status, "200".to_string());
        assert_eq!(actual_explanation, "OK".to_string());
        assert_ne!(actual_body, "".to_string());
    }

    //#[test] TODO: this isn't working
    fn test_request_google() {
        let actual = request("google.com", "", "");
        let actual_version = actual.0;
        let actual_status = actual.1;
        let actual_explanation = actual.2;
        let actual_body = actual.4;

        assert_eq!(actual_version, "HTTP/1.0".to_string());
        assert_eq!(actual_status, "200".to_string());
        assert_eq!(actual_explanation, "OK".to_string());
        assert_ne!(actual_body, "".to_string());
    }

    #[test]
    fn test_show() {
        let test_body = "<body>Test text.</body>";
        let actual = show(test_body);
        let expected = "Test text.";
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn test_show_empty() {
        let test_body = "";
        show(test_body);
    }
}