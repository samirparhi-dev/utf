def calculate_sum(a, b):
    """Calculate the sum of two numbers"""
    return a + b

def validate_email(email):
    """Validate email format"""
    if not email or '@' not in email:
        return False
    return True

class UserAccount:
    def __init__(self, username, email):
        self.username = username
        self.email = email
    
    def is_valid(self):
        return len(self.username) > 0 and validate_email(self.email)