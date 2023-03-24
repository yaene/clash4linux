const ADMINISTRATOR_URL: &str = "http://localhost:9090";

pub fn read_proxies() -> Result<String, reqwest::Error> {
    reqwest::blocking::get(ADMINISTRATOR_URL.to_string() + "/proxies")?.text()
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
}
