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
    
    private void validateInput(int value) {
        if (value < 0) {
            throw new IllegalArgumentException("Value cannot be negative");
        }
    }
}