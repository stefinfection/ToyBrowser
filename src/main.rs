mod network;

fn main() {
    let test_url: &str = "http://example.org/index.html";
    let url_pieces = network::parse_url(test_url, false);
    network::request(&url_pieces.0, &url_pieces.1, &url_pieces.2);

}




