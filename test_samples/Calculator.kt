class Calculator {
    
    fun add(a: Int, b: Int): Int {
        return a + b
    }
    
    fun multiply(x: Double, y: Double): Double {
        return x * y
    }
    
    fun isEven(number: Int): Boolean {
        return number % 2 == 0
    }
}

fun main() {
    val calc = Calculator()
    println("Testing Kotlin calculator")
    println("add(5, 3) = ${calc.add(5, 3)}")
    println("multiply(2.5, 4.0) = ${calc.multiply(2.5, 4.0)}")
    println("isEven(6) = ${calc.isEven(6)}")
}