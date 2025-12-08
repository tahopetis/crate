import { ApiResponse, PaginatedResponse } from '@/lib/types';

// API Configuration
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

// API Client class
class ApiClient {
  private baseURL: string;
  private defaultHeaders: Record<string, string>;

  constructor(baseURL: string = API_BASE_URL) {
    this.baseURL = baseURL;
    this.defaultHeaders = {
      'Content-Type': 'application/json',
    };
  }

  private getAuthHeader(): Record<string, string> {
    if (typeof window === 'undefined') return {};

    try {
      const authStorage = localStorage.getItem('auth-storage');
      if (authStorage) {
        const { state } = JSON.parse(authStorage);
        if (state.token) {
          return { Authorization: `Bearer ${state.token}` };
        }
      }
    } catch (error) {
      console.error('Error retrieving auth token:', error);
    }

    return {};
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseURL}${endpoint}`;
    const headers = {
      ...this.defaultHeaders,
      ...this.getAuthHeader(),
      ...options.headers,
    };

    // Handle body serialization
    let body = options.body;
    if (body && typeof body === 'object' && !(body instanceof FormData)) {
      body = JSON.stringify(body);
    }

    try {
      const response = await fetch(url, {
        ...options,
        headers,
        body,
      });

      // Handle 401 Unauthorized
      if (response.status === 401) {
        // Clear local storage and redirect to login
        localStorage.removeItem('auth-storage');
        if (typeof window !== 'undefined' && !window.location.pathname.includes('/auth')) {
          window.location.href = '/auth/login';
        }
        throw new Error('Session expired. Please log in again.');
      }

      // Handle other HTTP errors
      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        const message = errorData.message || errorData.error || `HTTP ${response.status}: ${response.statusText}`;
        throw new Error(message);
      }

      // Handle empty responses
      const contentType = response.headers.get('content-type');
      if (!contentType || !contentType.includes('application/json')) {
        return {} as T;
      }

      return response.json();
    } catch (error) {
      if (error instanceof Error) {
        throw error;
      }
      throw new Error('Network error occurred');
    }
  }

  // HTTP Methods
  async get<T>(endpoint: string, params?: Record<string, any>): Promise<T> {
    let url = endpoint;
    if (params) {
      const searchParams = new URLSearchParams();
      Object.entries(params).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          searchParams.append(key, String(value));
        }
      });
      const queryString = searchParams.toString();
      if (queryString) {
        url += `?${queryString}`;
      }
    }

    return this.request<T>(url, { method: 'GET' });
  }

  async post<T>(endpoint: string, data?: any): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'POST',
      body: data,
    });
  }

  async put<T>(endpoint: string, data?: any): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'PUT',
      body: data,
    });
  }

  async patch<T>(endpoint: string, data?: any): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'PATCH',
      body: data,
    });
  }

  async delete<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'DELETE' });
  }

  // File upload
  async upload<T>(endpoint: string, file: File, additionalData?: Record<string, any>): Promise<T> {
    const formData = new FormData();
    formData.append('file', file);

    if (additionalData) {
      Object.entries(additionalData).forEach(([key, value]) => {
        formData.append(key, String(value));
      });
    }

    return this.request<T>(endpoint, {
      method: 'POST',
      body: formData,
      headers: {}, // Let browser set Content-Type for FormData
    });
  }
}

// Create and export API client instance
export const apiClient = new ApiClient();

// API endpoints
export const apiEndpoints = {
  // Authentication
  auth: {
    login: '/auth/login',
    register: '/auth/register',
    logout: '/auth/logout',
    refresh: '/auth/refresh',
    me: '/auth/me',
  },

  // CI Management
  ci: {
    types: '/ci/types',
    lifecycles: '/ci/lifecycles',
    assets: '/ci/assets',
    relationshipTypes: '/ci/relationship-types',
    relationships: '/ci/relationships',
    import: '/ci/import',
    export: '/ci/export',
  },

  // Graph
  graph: {
    data: '/graph/data',
    layout: '/graph/layout',
    search: '/graph/search',
  },

  // Audit
  audit: {
    logs: '/audit/logs',
    changes: '/audit/changes',
  },

  // Amortization
  amortization: {
    calculations: '/amortization/calculations',
    history: '/amortization/history',
    reports: '/amortization/reports',
  },

  // Dashboard
  dashboard: {
    stats: '/dashboard/stats',
    activity: '/dashboard/activity',
    topAssets: '/dashboard/top-assets',
  },
} as const;

// API hooks and utilities
export const createApiHook = <T>(endpoint: string, options?: RequestInit) => {
  return async (params?: Record<string, any>): Promise<T> => {
    let url = endpoint;
    if (params) {
      const searchParams = new URLSearchParams();
      Object.entries(params).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          searchParams.append(key, String(value));
        }
      });
      const queryString = searchParams.toString();
      if (queryString) {
        url += `?${queryString}`;
      }
    }

    return apiClient.request<T>(url, options);
  };
};

// Error handling utility
export const handleApiError = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }
  return 'An unexpected error occurred';
};

// Response type guards
export const isPaginatedResponse = <T>(response: any): response is PaginatedResponse<T> => {
  return response && typeof response === 'object' && 'data' in response && 'total' in response;
};

export const getPaginationData = <T>(response: any): PaginatedResponse<T> => {
  if (isPaginatedResponse<T>(response)) {
    return response;
  }
  return {
    data: Array.isArray(response) ? response : [response],
    total: Array.isArray(response) ? response.length : 1,
    page: 1,
    limit: 10,
    totalPages: 1,
  };
};