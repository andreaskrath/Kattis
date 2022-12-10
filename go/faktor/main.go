// Faktor
// https://open.kattis.com/problems/faktor

package main

import (
	"fmt"
	"math"
)

func main() {
	var articles, impactFactor int

	fmt.Scanf("%d %d", &articles, &impactFactor)

	var scientists int
	for true {
		if math.Ceil(float64(scientists)/float64(articles)) == float64(impactFactor) {
			break
		}
		scientists++
	}
	fmt.Println(scientists)
}

// input = x y
// x number of articles
// y desired impact factor

// impact factor is calculated as:
// nr of citations/number of articles
