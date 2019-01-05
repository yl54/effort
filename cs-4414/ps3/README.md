# ps2 and ps3

This folder contains a combination of homework ps2 and ps3. 

* ps2 is in `src/gash/`.
* ps3 is in `src/server/`.

## Benchmarking the Webserver

Building a webserver that can handle many concurrent requests is extremely valuable. As a result, it is important to continuously benchmark the webserver's performance. One program is `hey`, which can be found in this [Github repo](https://github.com/rakyll/hey). An example usage is:
```
hey -n 500 -c 500 -m GET http://localhost:20001
```