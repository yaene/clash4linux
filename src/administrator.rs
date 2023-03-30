// RUN WITH cargo test -- --nocapture
// to see output for current integration tests
// todo yb: make proper unit tests and error handling + cleanup code - lots to do :(

const ADMINISTRATOR_URL: &str = "http://localhost:9090";

fn read_proxies() -> Result<reqwest::blocking::Response, reqwest::Error> {
    reqwest::blocking::get(ADMINISTRATOR_URL.to_string() + "/proxies")
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
pub fn read_selectors() -> Result<Vec<String>, reqwest::Error> {
    let proxies = &read_proxies()?.json::<serde_json::Value>()?["proxies"];

    let proxies: Option<&serde_json::Map<String, serde_json::Value>> =
        proxies.as_object();

    Ok(proxies
        .expect("unexpected response format, check code")
        .values()
        .filter(|proxy| proxy["type"] == "Selector")
        .filter_map(|proxy| proxy["name"].as_str())
        .map(String::from)
        .collect::<Vec<String>>())
}

pub fn read_proxies_for_selector(
    selector: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let proxies = &read_proxies()?.json::<serde_json::Value>()?["proxies"]
        [selector]["all"];

    let proxies = proxies.as_array();

    Ok(proxies
        .expect("unexpected response format, check code")
        .iter()
        .filter_map(serde_json::Value::as_str)
        .map(String::from)
        .collect::<Vec<String>>())
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
        match use_proxy("🚀 节点选择", "荷兰｜IEPL｜欧洲-2X") {
            Ok(resp) => {
                println!("{:#?}", resp);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }
    #[test]

    fn test_read_selectors() {
        match read_selectors() {
            Ok(resp) => {
                println!("{:#?}", resp);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }

    #[test]
    fn test_read_proxies_for_selector() {
        match read_proxies_for_selector("🚀 节点选择") {
            Ok(resp) => {
                println!("{:#?}", resp);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }
}
