#!/usr/bin/env python3
"""
Demo script showcasing the improved test generation capabilities
"""

def validate_email(email):
    """Validate email format using regex"""
    import re
    pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
    return bool(re.match(pattern, email))

def calculate_area(length, width):
    """Calculate area with validation"""
    if not isinstance(length, (int, float)) or not isinstance(width, (int, float)):
        raise TypeError("Length and width must be numbers")
    
    if length < 0 or width < 0:
        raise ValueError("Dimensions cannot be negative")
    
    return length * width

async def fetch_user_data(user_id):
    """Async function to fetch user data from API"""
    import aiohttp
    import asyncio
    
    if not user_id:
        raise ValueError("User ID is required")
    
    async with aiohttp.ClientSession() as session:
        async with session.get(f'https://api.example.com/users/{user_id}') as response:
            if response.status == 404:
                raise ValueError("User not found")
            elif response.status != 200:
                raise RuntimeError(f"API error: {response.status}")
            
            return await response.json()

class UserManager:
    """User management class with database operations"""
    
    def __init__(self):
        self.users = {}
        self.cache = {}
    
    def create_user(self, user_data):
        """Create a new user with validation"""
        if not user_data.get('email'):
            raise ValueError("Email is required")
        
        if not validate_email(user_data['email']):
            raise ValueError("Invalid email format")
        
        user_id = len(self.users) + 1
        user_data['id'] = user_id
        self.users[user_id] = user_data
        
        return user_data
    
    def get_user(self, user_id):
        """Get user with caching"""
        if user_id in self.cache:
            return self.cache[user_id]
        
        user = self.users.get(user_id)
        if user:
            self.cache[user_id] = user
        
        return user
    
    def delete_user(self, user_id):
        """Delete user and clear cache"""
        if user_id in self.users:
            del self.users[user_id]
        
        if user_id in self.cache:
            del self.cache[user_id]
        
        return True

def process_file(file_path):
    """Process file with error handling"""
    try:
        with open(file_path, 'r') as file:
            content = file.read()
            
        # Process content
        lines = content.split('\n')
        processed_lines = [line.strip() for line in lines if line.strip()]
        
        return {
            'total_lines': len(lines),
            'processed_lines': len(processed_lines),
            'content_preview': processed_lines[:5]
        }
    
    except FileNotFoundError:
        raise FileNotFoundError(f"File not found: {file_path}")
    except PermissionError:
        raise PermissionError(f"Permission denied: {file_path}")
    except Exception as e:
        raise RuntimeError(f"Error processing file: {str(e)}")

def authenticate_user(username, password):
    """Authenticate user with security considerations"""
    if not username or not password:
        return {"success": False, "error": "Username and password required"}
    
    # Simulate database lookup (with protection against injection)
    safe_username = username.replace("'", "''")  # Basic SQL injection protection
    
    # Mock authentication logic
    if username == "admin" and password == "secure_password_123":
        return {
            "success": True,
            "user_id": 1,
            "role": "admin",
            "token": "jwt_token_here"
        }
    
    return {"success": False, "error": "Invalid credentials"}

# Example usage
if __name__ == "__main__":
    # Email validation examples
    print("Email validation tests:")
    print(f"Valid email: {validate_email('user@example.com')}")
    print(f"Invalid email: {validate_email('invalid-email')}")
    
    # Area calculation examples
    print(f"\nArea calculation: {calculate_area(5, 3)}")
    
    # User management examples
    user_manager = UserManager()
    user = user_manager.create_user({
        "name": "John Doe",
        "email": "john@example.com"
    })
    print(f"\nCreated user: {user}")
    
    # Authentication examples
    auth_result = authenticate_user("admin", "secure_password_123")
    print(f"\nAuthentication result: {auth_result}")