package main

import "fmt"

func main() {
	var input, input2 string

	fmt.Scanf("%s %s", &input, &input2)

	if input+" "+input2 == "OCT 31" || input+" "+input2 == "DEC 25" {
		fmt.Println("yup")
	} else {
		fmt.Println("nope")
	}
}
