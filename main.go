package main

import (
	"html/template"
	"log"
	"net/http"
)

func main() {
	index := template.Must(template.ParseFiles("templates/index.html", "templates/hello.html"))

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		index.Execute(w, nil)
	})

	http.HandleFunc("/styles.css", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "assets/styles.css")
	})

	http.HandleFunc("/click", func(w http.ResponseWriter, r *http.Request) {
        w.Write([]byte("Success"))
	})

	http.HandleFunc("/htmx.min.js", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "assets/htmx.min.js")
	})

	log.Println("starting server!")
	http.ListenAndServe(":8080", nil)
}
