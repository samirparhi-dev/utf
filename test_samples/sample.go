package main

import "fmt"

func Add(a int, b int) int {
    return a + b
}

func Multiply(x float64, y float64) float64 {
    return x * y
}

func IsEven(n int) bool {
    return n%2 == 0
}

func main() {
    fmt.Println("Testing Go functions")
    fmt.Printf("Add(2, 3) = %d\n", Add(2, 3))
    fmt.Printf("Multiply(2.5, 4.0) = %f\n", Multiply(2.5, 4.0))
    fmt.Printf("IsEven(4) = %t\n", IsEven(4))
}