// Echo Echo Echo
// https://open.kattis.com/problems/echoechoecho
package main

import "fmt"

func main() {
	var inputString string

	fmt.Scanln(&inputString)

	outputString := inputString + " " + inputString + " " + inputString
	fmt.Println(outputString)
}
