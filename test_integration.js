import React from 'react';
import axios from 'axios';
import { User } from './models/User';

// API Integration example
export const fetchUsers = async () => {
    const response = await fetch('/api/users');
    return response.json();
};

export const createUser = async (userData) => {
    return await axios.post('/api/users', userData, {
        headers: {
            'Authorization': 'Bearer token123'
        }
    });
};

export const deleteUser = async (userId) => {
    return await axios.delete(`/api/users/${userId}`);
};

// Database Operations example
export class UserService {
    async createUser(data) {
        return await User.create(data);
    }

    async findUser(id) {
        return await User.findById(id);
    }

    async updateUser(id, data) {
        return await User.update(data, { where: { id } });
    }
}

// React Component example
export default function UserProfile({ userId, onUpdate }) {
    const [user, setUser] = React.useState(null);
    
    React.useEffect(() => {
        fetchUsers().then(users => {
            const currentUser = users.find(u => u.id === userId);
            setUser(currentUser);
        });
    }, [userId]);

    const handleUpdate = async (newData) => {
        await createUser(newData);
        onUpdate();
    };

    return (
        <div>
            <h1>{user?.name}</h1>
            <button onClick={handleUpdate}>Update</button>
        </div>
    );
}