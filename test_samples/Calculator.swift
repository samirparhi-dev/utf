import Foundation

class Calculator {
    
    func add(a: Int, b: Int) -> Int {
        return a + b
    }
    
    func multiply(x: Double, y: Double) -> Double {
        return x * y
    }
    
    func isEven(number: Int) -> Bool {
        return number % 2 == 0
    }
}

func main() {
    let calc = Calculator()
    print("Testing Swift calculator")
    print("add(5, 3) = \(calc.add(a: 5, b: 3))")
    print("multiply(2.5, 4.0) = \(calc.multiply(x: 2.5, y: 4.0))")
    print("isEven(6) = \(calc.isEven(number: 6))")
}