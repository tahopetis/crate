import React from 'react';
import { NextRequest, NextResponse } from 'next/server';
import { User, LoginCredentials, RegisterData } from '@/lib/types';

// Authentication utilities
export const authUtils = {
  // Token management
  getToken: (): string | null => {
    if (typeof window === 'undefined') return null;

    try {
      const authStorage = localStorage.getItem('auth-storage');
      if (authStorage) {
        const { state } = JSON.parse(authStorage);
        return state.token || null;
      }
    } catch (error) {
      console.error('Error retrieving auth token:', error);
    }
    return null;
  },

  setToken: (token: string): void => {
    if (typeof window === 'undefined') return;

    try {
      const authStorage = localStorage.getItem('auth-storage');
      const current = authStorage ? JSON.parse(authStorage) : { state: {} };
      current.state.token = token;
      current.state.isAuthenticated = !!token;
      localStorage.setItem('auth-storage', JSON.stringify(current));
    } catch (error) {
      console.error('Error setting auth token:', error);
    }
  },

  removeToken: (): void => {
    if (typeof window === 'undefined') return;

    try {
      const authStorage = localStorage.getItem('auth-storage');
      if (authStorage) {
        const current = JSON.parse(authStorage);
        current.state.token = null;
        current.state.isAuthenticated = false;
        current.state.user = null;
        localStorage.setItem('auth-storage', JSON.stringify(current));
      }
    } catch (error) {
      console.error('Error removing auth token:', error);
    }
  },

  // User management
  getUser: (): User | null => {
    if (typeof window === 'undefined') return null;

    try {
      const authStorage = localStorage.getItem('auth-storage');
      if (authStorage) {
        const { state } = JSON.parse(authStorage);
        return state.user || null;
      }
    } catch (error) {
      console.error('Error retrieving user data:', error);
    }
    return null;
  },

  setUser: (user: User): void => {
    if (typeof window === 'undefined') return;

    try {
      const authStorage = localStorage.getItem('auth-storage');
      const current = authStorage ? JSON.parse(authStorage) : { state: {} };
      current.state.user = user;
      localStorage.setItem('auth-storage', JSON.stringify(current));
    } catch (error) {
      console.error('Error setting user data:', error);
    }
  },

  // Authentication state
  isAuthenticated: (): boolean => {
    return !!authUtils.getToken() && !!authUtils.getUser();
  },

  clearAuth: (): void => {
    if (typeof window === 'undefined') return;

    try {
      localStorage.removeItem('auth-storage');
    } catch (error) {
      console.error('Error clearing auth data:', error);
    }
  },

  // Token validation
  isTokenExpired: (token: string): boolean => {
    try {
      const payload = JSON.parse(atob(token.split('.')[1]));
      const currentTime = Date.now() / 1000;
      return payload.exp < currentTime;
    } catch (error) {
      return true; // If we can't parse the token, assume it's expired
    }
  },

  // Role-based access
  hasRole: (user: User | null, role: string): boolean => {
    return user?.role === role;
  },

  hasAnyRole: (user: User | null, roles: string[]): boolean => {
    return user ? roles.includes(user.role) : false;
  },

  isAdmin: (user: User | null): boolean => {
    return authUtils.hasRole(user, 'admin');
  },

  // Password validation
  validatePassword: (password: string): { isValid: boolean; errors: string[] } => {
    const errors: string[] = [];

    if (password.length < 8) {
      errors.push('Password must be at least 8 characters long');
    }

    if (!/[A-Z]/.test(password)) {
      errors.push('Password must contain at least one uppercase letter');
    }

    if (!/[a-z]/.test(password)) {
      errors.push('Password must contain at least one lowercase letter');
    }

    if (!/\d/.test(password)) {
      errors.push('Password must contain at least one number');
    }

    if (!/[!@#$%^&*(),.?":{}|<>]/.test(password)) {
      errors.push('Password must contain at least one special character');
    }

    return {
      isValid: errors.length === 0,
      errors,
    };
  },

  // Email validation
  validateEmail: (email: string): boolean => {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
  },
};

// Middleware for server-side auth protection
export function withAuth(handler: (req: NextRequest, context: { user: User }) => Promise<NextResponse>) {
  return async (req: NextRequest): Promise<NextResponse> => {
    const token = req.headers.get('authorization')?.replace('Bearer ', '');

    if (!token) {
      return NextResponse.json({ error: 'No token provided' }, { status: 401 });
    }

    try {
      // Verify token with backend
      const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/auth/me`, {
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        return NextResponse.json({ error: 'Invalid token' }, { status: 401 });
      }

      const user = await response.json();
      return handler(req, { user });
    } catch (error) {
      return NextResponse.json({ error: 'Authentication failed' }, { status: 401 });
    }
  };
}

// HOC for client-side route protection
export function withAuthComponent<P extends object>(
  Component: React.ComponentType<P & { user: User }>
) {
  return function AuthenticatedComponent(props: P) {
    const user = authUtils.getUser();

    if (!user) {
      if (typeof window !== 'undefined') {
        window.location.href = '/auth/login';
      }
      return null;
    }

    return React.createElement(Component, { ...props, user } as any);
  };
}

// Auth context for React components
export interface AuthContextType {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  login: (credentials: LoginCredentials) => Promise<void>;
  register: (data: RegisterData) => Promise<void>;
  logout: () => void;
  loading: boolean;
}

// Utility functions for API requests with auth
export const createAuthenticatedRequest = async (endpoint: string, options: RequestInit = {}) => {
  const token = authUtils.getToken();

  const headers = {
    'Content-Type': 'application/json',
    ...(token && { Authorization: `Bearer ${token}` }),
    ...options.headers,
  };

  return fetch(`${process.env.NEXT_PUBLIC_API_URL}${endpoint}`, {
    ...options,
    headers,
  });
};

// Common auth API calls
export const authApi = {
  login: async (credentials: LoginCredentials): Promise<{ user: User; token: string }> => {
    const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(credentials),
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({}));
      throw new Error(error.message || 'Login failed');
    }

    return response.json();
  },

  register: async (data: RegisterData): Promise<{ user: User; token: string }> => {
    const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({}));
      throw new Error(error.message || 'Registration failed');
    }

    return response.json();
  },

  logout: async (): Promise<void> => {
    try {
      await createAuthenticatedRequest('/auth/logout', { method: 'POST' });
    } catch (error) {
      // Continue with logout even if API call fails
      console.error('Logout API call failed:', error);
    }
    authUtils.clearAuth();
  },

  getCurrentUser: async (): Promise<User> => {
    const response = await createAuthenticatedRequest('/auth/me');

    if (!response.ok) {
      throw new Error('Failed to get current user');
    }

    return response.json();
  },

  refreshToken: async (): Promise<string> => {
    const token = authUtils.getToken();
    if (!token) {
      throw new Error('No refresh token available');
    }

    const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/auth/refresh`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
      },
    });

    if (!response.ok) {
      throw new Error('Failed to refresh token');
    }

    const data = await response.json();
    authUtils.setToken(data.token);
    return data.token;
  },
};