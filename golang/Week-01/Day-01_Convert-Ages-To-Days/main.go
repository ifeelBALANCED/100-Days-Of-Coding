package main

import (
	"fmt"
	"os"
	"strconv"
)

func calcAge(age int) int {
	return age * 365
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run main.go <age>")
		return
	}

	age, err := strconv.Atoi(os.Args[1])
	if age <= 0 {
		fmt.Println("Age cannot be less or equal 0")
		return
	}
	if err != nil {
		fmt.Println("Invalid age:", err)
		return
	}
	fmt.Printf("Your age in days %d\n", calcAge(age))
}
