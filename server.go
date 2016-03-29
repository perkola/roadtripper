package main

import (
    "log"
    "encoding/xml"
    "net/http"
    "net/url"
    "io/ioutil"
)

func main() {
    fs := http.FileServer(http.Dir("public"))
    http.Handle("/", fs)

    log.Println("Listening...")
    /*cityA := "San Francisco"
    cityB := "Las Vegas"
    cities := []string{cityA, cityB}
    log.Printf("Dist from %s to %s is %.2f\n", cityA, cityB,
      DistCities(cities))*/
    http.ListenAndServe(":8000", nil)
}

type Response struct {
  XMLName xml.Name `"xml:response"`
  Distance float32 `"xml:distance"`
}

func DistCities(cities []string) float32 {

  u, err := url.Parse("http://mapquestapi.com/directions/v2/routematrix")
  if err != nil {
    log.Fatal(err)
  }
  q := u.Query()
  q.Set("key", "vOULz5vyrPWAgcHjh7M9Rd43rFzE3V5Z")
  q.Set("locations", "json:{locations:[San Francisco, Las Vegas]}")
  q.Set("outFormat", "xml")
  u.RawQuery = q.Encode()
  log.Printf("Api request to %s\n", u)
  resp, err := http.Get(u.String())
  if err != nil {
    log.Fatal(err)
  }
  defer resp.Body.Close()

  contents, err := ioutil.ReadAll(resp.Body)
  if err != nil {
    log.Fatal(err)
  }

  v := &Response{Distance:1.337}
  xml.Unmarshal(contents, &v)
  if err != nil {
    log.Fatal(err)
  }
  log.Printf("%s %.2f\n", v.XMLName, v.Distance)

  return 0.01
}
