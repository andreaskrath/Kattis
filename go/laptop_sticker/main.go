// Laptop Sticker
// https://open.kattis.com/problems/laptopsticker
package main

import "fmt"

func main() {
	var wc, hc, ws, hs int
	fmt.Scanf("%d %d %d %d", &wc, &hc, &ws, &hs)

	if (wc-1) > ws && (hc-1) > hs {
		fmt.Println(1)
	} else {
		fmt.Println(0)
	}
}
