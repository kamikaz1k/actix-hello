package main

import (
    "fmt"
    "encoding/json"
    "log"
    "net/http"
)

func main() {
    httpPort := 9000

    fmt.Printf("listening on %v\n", httpPort)

    http.HandleFunc("/", HelloServer)
    http.HandleFunc("/ping", PingServer)
    http.ListenAndServe(fmt.Sprintf(":%d", httpPort), logRequest(http.DefaultServeMux))
}

func PingServer(w http.ResponseWriter, r *http.Request) {
   // fmt.Fprintf(w, "pong");
  pong_instance := &Pong{Value: "pong"}
  pong_json_bytes, _ := json.Marshal(pong_instance)

  fmt.Fprintf(w, "%+v", string(pong_json_bytes));
}

func HelloServer(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintf(w, "Hello, %s!", r.URL.Path[1:])
}

func logRequest(handler http.Handler) http.Handler {

  return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
    log.Printf("%s %s %s\n", r.RemoteAddr, r.Method, r.URL)
    handler.ServeHTTP(w, r)
  })
}

type Pong struct {
  Value string `json:"value"`
}
