def calculate_area(length, width):
    """Calculate the area of a rectangle."""
    return length * width

def validate_email(email):
    """Validate email address format."""
    import re
    pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
    return re.match(pattern, email) is not None

class User:
    def __init__(self, email, name):
        self.email = email
        self.name = name