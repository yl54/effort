# ps2 and ps3

This folder contains a combination of homework ps2 and ps3. Used homework from the [University of Virginia](https://www.rust-class.org/).

* ps2 is in `src/gash/`. It contains a simple REPL GASH shell. 
* ps3 is in `src/server/`. It contains multiple versions of webservers.
    * `simple/` contains a simple, single-threaded webserver. It is the easiest version to understand and build.

## Benchmarking the Webserver

Building a webserver that can handle many concurrent requests is extremely important and valuable. As a result, it is important to continuously benchmark the webserver's performance. One program is `hey`, which can be found in this [Github repo](https://github.com/rakyll/hey). An example usage is:
```
hey -n 500 -c 500 -m GET http://localhost:20001
```