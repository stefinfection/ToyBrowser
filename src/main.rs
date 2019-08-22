mod network;

fn main() {
    let mut test_url: &str = "http://example.org";
    network::parse_url(test_url);

    test_url = "http://localhost:8080/";
    network::parse_url(test_url);

    test_url = "http://example.org/index.html#head";
    network::parse_url(test_url);
}




