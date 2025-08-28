function calculateSum(a, b) {
    return a + b;
}

function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

// HTML form with email input
const emailInput = '<input type="email" name="userEmail" required />';