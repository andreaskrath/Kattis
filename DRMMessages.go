package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	input := scanner.Bytes()

	s1, s2 := input[:len(input)/2], input[len(input)/2:]
	s1RotVal := sliceRotVal(s1)
	s2RotVal := sliceRotVal(s2)
	newS1 := sliceRot(s1, s1RotVal)
	newS2 := sliceRot(s2, s2RotVal)

	for i, v := range newS2 {
		newS1[i] = singleRot(newS1[i], singleRotVal(v))
	}
	fmt.Println(string(newS1))
}

func sliceRotVal(s []byte) int {
	var n int
	for _, v := range s {
		n += (int(v) - 65)
	}
	return n
}

func sliceRot(s []byte, rotVal int) []byte {
	for i, v := range s {
		s[i] = ((v - 65 + byte(rotVal)) % 26) + 65
	}
	return s
}

func singleRotVal(b byte) byte {
	return b - 65
}

func singleRot(b byte, r byte) byte {
	return ((b - 65 + r) % 26) + 65
}
