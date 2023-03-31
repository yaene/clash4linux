# Clash4Linux
A linux client for administrating [clash](https://github.com/Dreamacro/clash) proxy through their API.
Built with Qt and Rust.

### useful commands
[clash controller api reference](https://github.com/Dreamacro/clash/wiki/External-controller-API-Reference)
```bash
curl http://localhost:9090/proxies # get proxy list
curl -X PUT http://localhost:9090/proxies/[proxy selector name] -d '{"name":"[proxy name]"}' # set proxy for selector
```
## Disclaimer
Dirty code very limited functionality. At the moment just for me to select proxies more easily because i'm lazy to send web requests and I wanted to use Rust in a proj for the first time.
