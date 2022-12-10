// FizzBuzz
// https://open.kattis.com/problems/fizzbuzz
package main

import "fmt"

func main() {
	var firstNum, secondNum, loopValue int

	fmt.Scanf("%d %d %d", &firstNum, &secondNum, &loopValue)

	for i := 1; i <= loopValue; i++ {
		switch {
		case i%firstNum == 0 && i%secondNum == 0:
			fmt.Println("FizzBuzz")
		case i%firstNum == 0:
			fmt.Println("Fizz")
		case i%secondNum == 0:
			fmt.Println("Buzz")
		default:
			fmt.Println(i)
		}
	}
}
