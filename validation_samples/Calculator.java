// Java Calculator with various patterns for comprehensive testing
import java.util.*;
import java.util.regex.Pattern;

/**
 * Calculator class with comprehensive functionality for testing
 */
public class Calculator {
    private double result;
    private List<String> history;
    
    // Email validation pattern
    private static final Pattern EMAIL_PATTERN = 
        Pattern.compile("^[A-Za-z0-9+_.-]+@([A-Za-z0-9.-]+\\.[A-Za-z]{2,})$");
    
    public Calculator() {
        this.result = 0.0;
        this.history = new ArrayList<>();
    }
    
    /**
     * Adds two numbers and returns the result
     * @param a first number
     * @param b second number
     * @return sum of a and b
     * @throws IllegalArgumentException if inputs are invalid
     */
    public double add(double a, double b) throws IllegalArgumentException {
        if (Double.isNaN(a) || Double.isNaN(b)) {
            throw new IllegalArgumentException("NaN values are not allowed");
        }
        
        if (Double.isInfinite(a) || Double.isInfinite(b)) {
            throw new IllegalArgumentException("Infinite values are not allowed");
        }
        
        double result = a + b;
        this.history.add(String.format("%.2f + %.2f = %.2f", a, b, result));
        this.result = result;
        return result;
    }
    
    /**
     * Divides two numbers and returns the result
     * @param a dividend
     * @param b divisor
     * @return quotient of a and b
     * @throws IllegalArgumentException if inputs are invalid
     * @throws ArithmeticException if division by zero
     */
    public double divide(double a, double b) throws IllegalArgumentException, ArithmeticException {
        if (Double.isNaN(a) || Double.isNaN(b)) {
            throw new IllegalArgumentException("NaN values are not allowed");
        }
        
        if (b == 0.0) {
            throw new ArithmeticException("Division by zero is not allowed");
        }
        
        double result = a / b;
        this.history.add(String.format("%.2f / %.2f = %.2f", a, b, result));
        this.result = result;
        return result;
    }
    
    /**
     * Calculates the nth Fibonacci number
     * @param n the position in Fibonacci sequence
     * @return the nth Fibonacci number
     * @throws IllegalArgumentException if n is negative or too large
     */
    public long fibonacci(int n) throws IllegalArgumentException {
        if (n < 0) {
            throw new IllegalArgumentException("Input must be non-negative");
        }
        
        if (n > 92) { // Prevent overflow for long
            throw new IllegalArgumentException("Input too large for long type");
        }
        
        if (n <= 1) {
            return n;
        }
        
        long a = 0, b = 1;
        for (int i = 2; i <= n; i++) {
            long temp = a + b;
            a = b;
            b = temp;
        }
        return b;
    }
    
    /**
     * Validates email format
     * @param email email string to validate
     * @return true if email format is valid
     */
    public boolean validateEmail(String email) {
        if (email == null || email.trim().isEmpty()) {
            return false;
        }
        
        return EMAIL_PATTERN.matcher(email.trim()).matches();
    }
    
    /**
     * Gets a copy of calculation history
     * @return list of calculation history
     */
    public List<String> getHistory() {
        return new ArrayList<>(this.history);
    }
    
    /**
     * Clears the calculation history
     */
    public void clearHistory() {
        this.history.clear();
    }
    
    /**
     * Gets current result
     * @return current result value
     */
    public double getResult() {
        return this.result;
    }
    
    /**
     * Sets the result value
     * @param result new result value
     */
    public void setResult(double result) {
        this.result = result;
    }
    
    // Static utility methods
    
    /**
     * Calculates area of rectangle
     * @param width rectangle width
     * @param height rectangle height
     * @return area of rectangle
     * @throws IllegalArgumentException if dimensions are invalid
     */
    public static double calculateArea(double width, double height) throws IllegalArgumentException {
        if (width <= 0 || height <= 0) {
            throw new IllegalArgumentException("Width and height must be positive");
        }
        
        if (Double.isNaN(width) || Double.isNaN(height)) {
            throw new IllegalArgumentException("NaN values are not allowed");
        }
        
        return width * height;
    }
    
    /**
     * Formats amount as currency string
     * @param amount amount to format
     * @return formatted currency string
     */
    public static String formatCurrency(double amount) {
        if (Double.isNaN(amount) || Double.isInfinite(amount)) {
            return "Invalid amount";
        }
        
        return String.format("$%.2f", amount);
    }
    
    /**
     * Checks if a number is prime
     * @param n number to check
     * @return true if number is prime
     */
    public static boolean isPrime(int n) {
        if (n < 2) {
            return false;
        }
        if (n == 2) {
            return true;
        }
        if (n % 2 == 0) {
            return false;
        }
        
        for (int i = 3; i * i <= n; i += 2) {
            if (n % i == 0) {
                return false;
            }
        }
        return true;
    }
    
    /**
     * Validates password strength
     * @param password password to validate
     * @return true if password meets strength requirements
     */
    public static boolean validatePassword(String password) {
        if (password == null || password.length() < 8) {
            return false;
        }
        
        boolean hasUpper = password.chars().anyMatch(Character::isUpperCase);
        boolean hasLower = password.chars().anyMatch(Character::isLowerCase);
        boolean hasDigit = password.chars().anyMatch(Character::isDigit);
        boolean hasSpecial = password.chars().anyMatch(ch -> "!@#$%^&*()".indexOf(ch) >= 0);
        
        return hasUpper && hasLower && hasDigit && hasSpecial;
    }
}