package main

import "testing"

func TestCalcAge(t *testing.T) {
	testParameters := []struct {
		input    string
		expected string
	}{
		{"I am finding Nemo !", "I found Nemo at 4!"},
		{"Nemo is me", "I found Nemo at 1!"},
		{"I Nemo am", "I found Nemo at 2!"},
		{"Hello, world", "I can't find Nemo :("},
		{"Nemo's friend is Dory", "I can't find Nemo :("},
		{"Nemo Nemo Nemo", "I found Nemo at 1!"},
	}

	for _, test := range testParameters {
		result := findNemo(test.input)
		if result != test.expected {
			t.Errorf("Expected %s, got %s", test.expected, result)
		}
	}
}
