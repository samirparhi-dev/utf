"""
Python Calculator with various patterns for comprehensive testing
"""
import re
from typing import List, Optional, Union


class Calculator:
    def __init__(self):
        self.result: float = 0.0
        self.history: List[str] = []

    def add(self, a: Union[int, float], b: Union[int, float]) -> Union[int, float]:
        """Add two numbers and return the result"""
        if not isinstance(a, (int, float)) or not isinstance(b, (int, float)):
            raise TypeError("Both arguments must be numbers")
        
        result = a + b
        self.history.append(f"{a} + {b} = {result}")
        return result

    def divide(self, a: Union[int, float], b: Union[int, float]) -> float:
        """Divide two numbers and return the result"""
        if not isinstance(a, (int, float)) or not isinstance(b, (int, float)):
            raise TypeError("Both arguments must be numbers")
        
        if b == 0:
            raise ValueError("Division by zero is not allowed")
        
        result = a / b
        self.history.append(f"{a} / {b} = {result}")
        return result

    def validate_email(self, email: str) -> bool:
        """Validate email format"""
        if not isinstance(email, str):
            return False
        
        if not email.strip():
            return False
        
        pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
        return bool(re.match(pattern, email.strip()))

    def fibonacci(self, n: int) -> int:
        """Calculate fibonacci number"""
        if not isinstance(n, int):
            raise TypeError("Input must be an integer")
        
        if n < 0:
            raise ValueError("Input must be non-negative")
        
        if n <= 1:
            return n
        
        return self.fibonacci(n - 1) + self.fibonacci(n - 2)

    def get_history(self) -> List[str]:
        """Get calculation history"""
        return self.history.copy()

    def clear_history(self) -> bool:
        """Clear calculation history"""
        self.history.clear()
        return True


def calculate_area(width: Union[int, float], height: Union[int, float]) -> Union[int, float]:
    """Calculate area of rectangle"""
    if not isinstance(width, (int, float)) or not isinstance(height, (int, float)):
        raise TypeError("Width and height must be numbers")
    
    if width <= 0 or height <= 0:
        raise ValueError("Width and height must be positive")
    
    return width * height


def format_currency(amount: Union[int, float]) -> str:
    """Format number as currency"""
    if not isinstance(amount, (int, float)):
        return "Invalid amount"
    
    return f"${amount:.2f}"


def validate_password(password: str) -> bool:
    """Validate password strength"""
    if not isinstance(password, str):
        return False
    
    if len(password) < 8:
        return False
    
    has_upper = any(c.isupper() for c in password)
    has_lower = any(c.islower() for c in password)
    has_digit = any(c.isdigit() for c in password)
    
    return has_upper and has_lower and has_digit