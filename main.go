package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {
	// fileServer serves HTML files from Dir templates
	fileServer := http.FileServer(http.Dir("./templates"))
	http.Handle("/", fileServer)
	fmt.Println("Listening on port 8080")
	// starts server on port 8080 and logs errors if there are any
	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatal(err)
	}
	fmt.Println("Hello world")
}
