package main

import "testing"

func TestSolutionSampleOne(t *testing.T) {
	a := 10
	b := 4
	c := 1335

	actual := solution(a, b, c)
	expected := 1321

	if actual != expected {
		t.Fatalf("got: %d, expected: %d", actual, expected)
	}
}

func TestSolutionSampleTwo(t *testing.T) {
	a := 8
	b := 10
	c := 1000

	actual := solution(a, b, c)
	expected := 982

	if actual != expected {
		t.Fatalf("got: %d, expected: %d", actual, expected)
	}
}
