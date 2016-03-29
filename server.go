package main

import (
  "io"
  "net/http"
  "fmt"
)

func handleRoot(w http.ResponseWriter, r *http.Request) {
  io.WriteString(w, "Hello World!")
}

func main() {
  http.HandleFunc("/", handleRoot)
  http.ListenAndServe(":8000", nil)
  fmt.Println("Server is running")
}
