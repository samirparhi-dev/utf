// Go Calculator with various patterns for comprehensive testing
package main

import (
	"errors"
	"fmt"
	"math"
	"regexp"
	"strings"
)

// Calculator represents a calculator with history
type Calculator struct {
	Result  float64
	History []string
}

// NewCalculator creates a new calculator instance
func NewCalculator() *Calculator {
	return &Calculator{
		Result:  0,
		History: make([]string, 0),
	}
}

// Add performs addition and returns the result
func (c *Calculator) Add(a, b float64) (float64, error) {
	if math.IsNaN(a) || math.IsNaN(b) {
		return 0, errors.New("NaN values not allowed")
	}
	
	if math.IsInf(a, 0) || math.IsInf(b, 0) {
		return 0, errors.New("infinite values not allowed")
	}
	
	result := a + b
	c.History = append(c.History, fmt.Sprintf("%.2f + %.2f = %.2f", a, b, result))
	c.Result = result
	return result, nil
}

// Divide performs division and returns the result
func (c *Calculator) Divide(a, b float64) (float64, error) {
	if math.IsNaN(a) || math.IsNaN(b) {
		return 0, errors.New("NaN values not allowed")
	}
	
	if b == 0 {
		return 0, errors.New("division by zero is not allowed")
	}
	
	result := a / b
	c.History = append(c.History, fmt.Sprintf("%.2f / %.2f = %.2f", a, b, result))
	c.Result = result
	return result, nil
}

// Fibonacci calculates the nth Fibonacci number
func (c *Calculator) Fibonacci(n int) (int, error) {
	if n < 0 {
		return 0, errors.New("input must be non-negative")
	}
	
	if n > 46 { // Prevent overflow for int
		return 0, errors.New("input too large")
	}
	
	if n <= 1 {
		return n, nil
	}
	
	a, b := 0, 1
	for i := 2; i <= n; i++ {
		a, b = b, a+b
	}
	return b, nil
}

// GetHistory returns a copy of the calculation history
func (c *Calculator) GetHistory() []string {
	history := make([]string, len(c.History))
	copy(history, c.History)
	return history
}

// ClearHistory clears the calculation history
func (c *Calculator) ClearHistory() {
	c.History = c.History[:0]
}

// Standalone functions for additional testing

// CalculateArea calculates the area of a rectangle
func CalculateArea(width, height float64) (float64, error) {
	if width <= 0 || height <= 0 {
		return 0, errors.New("width and height must be positive")
	}
	
	if math.IsNaN(width) || math.IsNaN(height) {
		return 0, errors.New("NaN values not allowed")
	}
	
	return width * height, nil
}

// ValidateEmail validates an email address format
func ValidateEmail(email string) bool {
	if email == "" {
		return false
	}
	
	email = strings.TrimSpace(email)
	if len(email) < 5 { // Minimum: a@b.c
		return false
	}
	
	emailRegex := regexp.MustCompile(`^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`)
	return emailRegex.MatchString(email)
}

// FormatCurrency formats a number as currency
func FormatCurrency(amount float64) string {
	if math.IsNaN(amount) || math.IsInf(amount, 0) {
		return "Invalid amount"
	}
	
	return fmt.Sprintf("$%.2f", amount)
}

// IsPrime checks if a number is prime
func IsPrime(n int) bool {
	if n < 2 {
		return false
	}
	if n == 2 {
		return true
	}
	if n%2 == 0 {
		return false
	}
	
	for i := 3; i*i <= n; i += 2 {
		if n%i == 0 {
			return false
		}
	}
	return true
}

// Max returns the maximum of two integers
func Max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// Min returns the minimum of two integers  
func Min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	calc := NewCalculator()
	result, _ := calc.Add(10, 5)
	fmt.Printf("10 + 5 = %.2f\n", result)
	
	fmt.Printf("Is 17 prime? %t\n", IsPrime(17))
	fmt.Printf("Email valid? %t\n", ValidateEmail("test@example.com"))
}