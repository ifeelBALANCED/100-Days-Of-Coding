package main

import (
	"fmt"
	"os"
	"strings"
)

func findNemo(inputString string) (result string) {
	inputSlice := strings.Split(inputString, " ")
	for i, word := range inputSlice {
		if word == "Nemo" {
			return fmt.Sprintf("I found Nemo at %d!", i+1)
		}
	}
	return "I can't find Nemo :("
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run main.go <string>")
		return
	}

	inputSlice := os.Args[1:]
	inputString := strings.Join(inputSlice, " ")
	if len(inputString) == 0 {
		fmt.Println("Input cannot be less or equal 0")
		return
	}
	result := findNemo(inputString)
	fmt.Print(result)
}
