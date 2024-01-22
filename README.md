This is a repository to specifically for reproducing an issue in drachtio-server

## Files

* docker-compose-local.yaml : runs drachtio server with `--contact "sip:172.29.0.10;transport=udp"` 
* docker-compose-star.yaml  : runs drachtio server with `--contact "sip:*;transport=udp"`

* js-app.Dockerfile : container for the application logic for both proxy and destination

* package.json
* package-lock.json

* src : source files
* ttg : a small rust ua like app that fires an INVITE every x (configurable) amount of seconds at the proxy
* ua.Dockerfile : container for the INVITE firing ua app
* logs: some part of the docker logs that point out the issue

## Execution

* docker-compose -f docker-compose-local.yaml up --build
* docker-compose -f docker-compose-star.yaml up --build


## logs

* star.out line 116 `drsip-destination-srf-1  | INVITE sip:+happy_feet@172.29.0.11;transport=UDP SIP/2.0`
* local.out line 117 `INVITE sip:+443332222@127.0.0.1:5060 SIP/2.0`

