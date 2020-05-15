# Web Server Benchmarks

```ascii
Y88b         /           888       ,d88~~\                                                   888~~\                             888                                     888   _
 Y88b       /   e88~~8e  888-~88e  8888     e88~~8e  888-~\ Y88b    /  e88~~8e  888-~\       888   |  e88~~8e  888-~88e  e88~~\ 888-~88e 888-~88e-~88e   /~~~8e  888-~\ 888 e~ ~   d88~\
  Y88b  e  /   d888  88b 888  888b `Y88b   d888  88b 888     Y88b  /  d888  88b 888          888 _/  d888  88b 888  888 d888    888  888 888  888  888       88b 888    888d8b    C888
   Y88bd8b/    8888__888 888  8888  `Y88b, 8888__888 888      Y88b/   8888__888 888          888  \  8888__888 888  888 8888    888  888 888  888  888  e88~-888 888    888Y88b    Y88b
    Y88Y8Y     Y888    , 888  888P    8888 Y888    , 888       Y8/    Y888    , 888          888   | Y888    , 888  888 Y888    888  888 888  888  888 C888  888 888    888 Y88b    888D
     Y  Y       "88___/  888-_88"  \__88P'  "88___/  888        Y      "88___/  888          888__/   "88___/  888  888  "88__/ 888  888 888  888  888  "88_-888 888    888  Y88b \_88P

```

## How to use

run any of the following make commands to run a particular type of web server

```bash
help                           Show all the available make commands
rust-raw                       run raw rust http
actix                          run actix-web http server
go-net                         raw go net/http server
go-gin                         run go gin server
gunicorn-flask                 run simple flask/gunicorn server
run-benchmark                  run wrk for benchmarking
```

then in another shell run the wrk command to benchmark it

```bash
$ make run-benchmark PORT=5000
Running 1s test @ http://127.0.0.1:5000/ping
  100 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    65.50ms   10.92ms  98.13ms   76.60%
    Req/Sec    14.30      5.12    20.00     40.27%
  1547 requests in 1.10s, 255.32KB read
Requests/sec:   1401.81
Transfer/sec:    231.35KB
Results for port 5000: gunicorn-flask
```
