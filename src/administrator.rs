// RUN WITH cargo test -- --nocapture
// to see output for current integration tests
// todo yb: make proper unit tests

const ADMINISTRATOR_URL: &str = "http://localhost:9090";

pub fn read_proxies() -> Result<String, reqwest::Error> {
    reqwest::blocking::get(ADMINISTRATOR_URL.to_string() + "/proxies")?.text()
}

pub fn use_proxy(
    selector: &str,
    proxy: &str,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client
        .put(ADMINISTRATOR_URL.to_string() + "/proxies/" + selector)
        .body(format!("{{\"name\":\"{}\"}}", proxy))
        .send()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_proxies() {
        match read_proxies() {
            Ok(resp) => {
                println!("{:#?}", resp);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }

    #[test]
    fn test_use_proxy() {
        match use_proxy(
            "%F0%9F%90%9F%20%E6%BC%8F%E7%BD%91%E4%B9%8B%E9%B1%BC",
            "荷兰｜IEPL｜欧洲-2X",
        ) {
            Ok(resp) => {
                println!("{:#?}", resp);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }
}
