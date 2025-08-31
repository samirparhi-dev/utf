// JavaScript Calculator with various patterns for comprehensive testing
class Calculator {
    constructor() {
        this.result = 0;
        this.history = [];
    }

    add(a, b) {
        if (typeof a !== 'number' || typeof b !== 'number') {
            throw new Error('Invalid input: both arguments must be numbers');
        }
        const result = a + b;
        this.history.push(`${a} + ${b} = ${result}`);
        return result;
    }

    divide(a, b) {
        if (typeof a !== 'number' || typeof b !== 'number') {
            throw new Error('Invalid input: both arguments must be numbers');
        }
        if (b === 0) {
            throw new Error('Division by zero is not allowed');
        }
        const result = a / b;
        this.history.push(`${a} / ${b} = ${result}`);
        return result;
    }

    validateEmail(email) {
        if (!email || typeof email !== 'string') {
            return false;
        }
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email.trim());
    }

    fibonacci(n) {
        if (typeof n !== 'number' || n < 0 || !Number.isInteger(n)) {
            throw new Error('Input must be a non-negative integer');
        }
        if (n <= 1) return n;
        return this.fibonacci(n - 1) + this.fibonacci(n - 2);
    }

    getHistory() {
        return [...this.history];
    }

    clearHistory() {
        this.history = [];
        return true;
    }
}

// Standalone functions for additional testing
function calculateArea(width, height) {
    if (width <= 0 || height <= 0) {
        throw new Error('Width and height must be positive numbers');
    }
    return width * height;
}

function formatCurrency(amount) {
    if (typeof amount !== 'number') {
        return 'Invalid amount';
    }
    return `$${amount.toFixed(2)}`;
}

module.exports = { Calculator, calculateArea, formatCurrency };