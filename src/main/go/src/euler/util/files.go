// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems

package util

import (
	"log"
	"os"
	"path/filepath"
	"strings"
)

func OpenResource(name string) *os.File {
	var basePath string
	if path, ok := os.LookupEnv("GOPATH"); ok {
		basePath = strings.Split(path, ":")[0]
	}
	absolute, _ := filepath.Abs(basePath)
	if file, err := os.Open(filepath.Join(absolute, "src", "euler",  "resources", name)); err != nil {
		log.Fatal(err)
		return file
	} else {
		return file
	}
}
