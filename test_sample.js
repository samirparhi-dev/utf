function calculateSum(a, b) {
    return a + b;
}

function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

class Calculator {
    add(x, y) {
        return x + y;
    }
    
    multiply(x, y) {
        return x * y;
    }
}

module.exports = { calculateSum, validateEmail, Calculator };