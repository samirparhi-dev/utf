public class Calculator {
    
    public int add(int a, int b) {
        return a + b;
    }
    
    public double multiply(double x, double y) {
        return x * y;
    }
    
    public boolean isEven(int number) {
        return number % 2 == 0;
    }
    
    public static void main(String[] args) {
        Calculator calc = new Calculator();
        System.out.println("Testing Java calculator");
        System.out.println("add(5, 3) = " + calc.add(5, 3));
        System.out.println("multiply(2.5, 4.0) = " + calc.multiply(2.5, 4.0));
        System.out.println("isEven(6) = " + calc.isEven(6));
    }
}