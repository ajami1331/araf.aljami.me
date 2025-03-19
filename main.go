package main

import (
	"net/http"
	"os"
)

const dest = "./dist"
const src = "./src"
const port = "3000"
const siteTitle = "Araf Al Jami"
const siteDescription = "Araf Al Jami's personal blog"
const githubRepo = "ajami1331/araf.aljami.me"

func build() {
	os.RemoveAll(dest)
	os.Mkdir(dest, 0755)
}

func main() {
	build()
	http.Handle("/", http.FileServer(http.Dir(dest)))
	http.ListenAndServe(":3000", nil)
}
