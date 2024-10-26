// api.ts
import axios from 'axios';

export const api = axios.create({
  baseURL: 'http://localhost:8080', // URL del backend
  headers: {
    'Content-Type': 'application/json',
  },
});

export const getUsers = () => api.get('/users');
export const createUser = (data: any) => api.post('/users', data);

