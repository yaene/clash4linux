# Clash4Linux
A linux client for administrating [clash](https://github.com/Dreamacro/clash) proxy through their API

### useful commands
[api reference](https://github.com/Dreamacro/clash/wiki/External-controller-API-Reference)
```bash
curl http://localhost:9090/proxies # get proxy list
curl -X PUT http://localhost:9090/proxies/[proxy selector name] -d '{"name":"[proxy name]"}' # set proxy for selector
```