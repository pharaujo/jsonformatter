package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	var ir interface{}
	var b []byte
	switch len(os.Args) {
	case 1:
		buf, err := ioutil.ReadAll(os.Stdin)
		if err != nil {
			fmt.Print(err)
			os.Exit(1)
		} else {
			b = buf
		}
	case 2:
		buf, err := ioutil.ReadFile(os.Args[1])
		if err != nil {
			fmt.Print(err)
			os.Exit(1)
		} else {
			b = buf
		}
	}
	err := json.Unmarshal(b, &ir)
	if err != nil {
		err := json.Unmarshal(b[strings.Index(string(b), "(")+1:len(b)-3], &ir)
		if err != nil {
			fmt.Print(err)
			os.Exit(1)
		}
	}
	json, _ := json.MarshalIndent(ir, "", "  ")
	fmt.Print(string(json))
}
