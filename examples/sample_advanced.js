/**
 * Advanced JavaScript functions for testing our improved template system
 */

// Email validation function
function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

// Async function for data fetching
async function fetchUserData(userId) {
    const response = await fetch(`/api/users/${userId}`);
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
}

// Mathematical calculation function
function calculateArea(length, width) {
    if (typeof length !== 'number' || typeof width !== 'number') {
        throw new TypeError('Both parameters must be numbers');
    }
    if (length < 0 || width < 0) {
        throw new Error('Dimensions cannot be negative');
    }
    return length * width;
}

// Class for user management
class UserManager {
    constructor() {
        this.users = new Map();
    }
    
    addUser(user) {
        if (!user || !user.id) {
            throw new Error('User must have an id');
        }
        this.users.set(user.id, user);
    }
    
    getUser(id) {
        return this.users.get(id);
    }
    
    getUserCount() {
        return this.users.size;
    }
}

// API endpoint simulation
function handleApiRequest(method, path, data) {
    if (method === 'GET' && path === '/api/health') {
        return { status: 'ok', timestamp: Date.now() };
    }
    
    if (method === 'POST' && path === '/api/users') {
        if (!data || !data.email) {
            return { error: 'Email is required', status: 400 };
        }
        return { 
            id: Math.random().toString(36).substr(2, 9),
            ...data,
            status: 201 
        };
    }
    
    return { error: 'Not found', status: 404 };
}

module.exports = {
    validateEmail,
    fetchUserData,
    calculateArea,
    UserManager,
    handleApiRequest
};