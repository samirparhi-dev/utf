// Rust Calculator with various patterns for comprehensive testing
use std::fmt;

#[derive(Debug, Clone)]
pub struct Calculator {
    pub result: f64,
    pub history: Vec<String>,
}

#[derive(Debug)]
pub enum CalculatorError {
    InvalidInput(String),
    DivisionByZero,
    NegativeInput,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CalculatorError::DivisionByZero => write!(f, "Division by zero is not allowed"),
            CalculatorError::NegativeInput => write!(f, "Negative input is not allowed"),
        }
    }
}

impl std::error::Error for CalculatorError {}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            result: 0.0,
            history: Vec::new(),
        }
    }

    pub fn add(&mut self, a: f64, b: f64) -> Result<f64, CalculatorError> {
        if a.is_nan() || b.is_nan() {
            return Err(CalculatorError::InvalidInput("NaN values not allowed".to_string()));
        }
        
        let result = a + b;
        self.history.push(format!("{} + {} = {}", a, b, result));
        self.result = result;
        Ok(result)
    }

    pub fn divide(&mut self, a: f64, b: f64) -> Result<f64, CalculatorError> {
        if a.is_nan() || b.is_nan() {
            return Err(CalculatorError::InvalidInput("NaN values not allowed".to_string()));
        }
        
        if b == 0.0 {
            return Err(CalculatorError::DivisionByZero);
        }
        
        let result = a / b;
        self.history.push(format!("{} / {} = {}", a, b, result));
        self.result = result;
        Ok(result)
    }

    pub fn fibonacci(&self, n: u32) -> Result<u64, CalculatorError> {
        if n > 93 {
            return Err(CalculatorError::InvalidInput("Input too large for u64".to_string()));
        }
        
        match n {
            0 => Ok(0),
            1 => Ok(1),
            _ => {
                let mut a = 0u64;
                let mut b = 1u64;
                for _ in 2..=n {
                    let temp = a.checked_add(b)
                        .ok_or(CalculatorError::InvalidInput("Overflow occurred".to_string()))?;
                    a = b;
                    b = temp;
                }
                Ok(b)
            }
        }
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

pub fn calculate_area(width: f64, height: f64) -> Result<f64, CalculatorError> {
    if width <= 0.0 || height <= 0.0 {
        return Err(CalculatorError::NegativeInput);
    }
    
    if width.is_nan() || height.is_nan() {
        return Err(CalculatorError::InvalidInput("NaN values not allowed".to_string()));
    }
    
    Ok(width * height)
}

pub fn validate_email(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }
    
    let email = email.trim();
    if email.len() < 5 {  // Minimum: a@b.c
        return false;
    }
    
    email.contains('@') && email.contains('.') && !email.starts_with('@') && !email.ends_with('@')
}

pub fn format_currency(amount: f64) -> String {
    if amount.is_nan() || amount.is_infinite() {
        return "Invalid amount".to_string();
    }
    
    format!("${:.2}", amount)
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}