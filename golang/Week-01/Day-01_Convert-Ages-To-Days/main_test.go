package main

import "testing"

func TestCalcAge(t *testing.T) {
	testParameters := []struct {
		age      int
		expected int
	}{
		{0, 0},
		{1, 365},
		{2, 730},
		{65, 23725},
		{20, 7300},
	}

	for _, test := range testParameters {
		result := calcAge(test.age)
		if result != test.expected {
			t.Errorf("Expected %d, got %d", test.expected, result)
		}
	}
}
