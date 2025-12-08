import { useState, useEffect, useCallback } from 'react';
import { apiClient } from '@/lib/api';
import { useUIStore } from '@/store/ui-store';
import { authUtils } from '@/lib/auth';

interface UseApiOptions {
  immediate?: boolean;
  onSuccess?: (data: any) => void;
  onError?: (error: Error) => void;
  retryCount?: number;
  retryDelay?: number;
}

interface UseApiState<T> {
  data: T | null;
  loading: boolean;
  error: Error | null;
  lastUpdated: Date | null;
}

export function useApi<T = any>(
  endpoint: string,
  options: UseApiOptions = {}
) {
  const {
    immediate = false,
    onSuccess,
    onError,
    retryCount = 0,
    retryDelay = 1000,
  } = options;

  const [state, setState] = useState<UseApiState<T>>({
    data: null,
    loading: false,
    error: null,
    lastUpdated: null,
  });

  const [retryAttempts, setRetryAttempts] = useState(0);

  const execute = useCallback(async (...args: any[]) => {
    setState((prev) => ({ ...prev, loading: true, error: null }));

    try {
      // Construct URL with query parameters if provided
      let url = endpoint;
      if (args.length > 0 && args[0]) {
        const params = new URLSearchParams();
        Object.entries(args[0]).forEach(([key, value]) => {
          if (value !== undefined && value !== null) {
            params.append(key, String(value));
          }
        });
        const queryString = params.toString();
        if (queryString) {
          url += `?${queryString}`;
        }
      }

      const data = await apiClient.get<T>(url);

      setState({
        data,
        loading: false,
        error: null,
        lastUpdated: new Date(),
      });

      setRetryAttempts(0);
      onSuccess?.(data);
      return data;
    } catch (error) {
      const err = error instanceof Error ? error : new Error('API request failed');

      // Retry logic
      if (retryAttempts < retryCount) {
        setTimeout(() => {
          setRetryAttempts((prev) => prev + 1);
          execute(...args);
        }, retryDelay);
      } else {
        setState((prev) => ({
          ...prev,
          loading: false,
          error: err,
        }));
        onError?.(err);
      }

      throw err;
    }
  }, [endpoint, onSuccess, onError, retryCount, retryDelay, retryAttempts]);

  // Auto-execute if immediate is true
  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [immediate, execute]);

  return {
    ...state,
    execute,
    reset: () => {
      setState({
        data: null,
        loading: false,
        error: null,
        lastUpdated: null,
      });
      setRetryAttempts(0);
    },
  };
}

// Hook for POST requests
export function useApiMutation<TData = any, TVariables = any>(
  endpoint: string,
  method: 'POST' | 'PUT' | 'PATCH' | 'DELETE' = 'POST',
  options: UseApiOptions = {}
) {
  const [state, setState] = useState<UseApiState<TData>>({
    data: null,
    loading: false,
    error: null,
    lastUpdated: null,
  });

  const { onSuccess, onError } = options;

  const mutate = useCallback(async (variables?: TVariables) => {
    setState((prev) => ({ ...prev, loading: true, error: null }));

    try {
      let data: TData;

      switch (method) {
        case 'POST':
          data = await apiClient.post<TData>(endpoint, variables);
          break;
        case 'PUT':
          data = await apiClient.put<TData>(endpoint, variables);
          break;
        case 'PATCH':
          data = await apiClient.patch<TData>(endpoint, variables);
          break;
        case 'DELETE':
          data = await apiClient.delete<TData>(endpoint);
          break;
        default:
          throw new Error(`Unsupported method: ${method}`);
      }

      setState({
        data,
        loading: false,
        error: null,
        lastUpdated: new Date(),
      });

      onSuccess?.(data);
      return data;
    } catch (error) {
      const err = error instanceof Error ? error : new Error('API request failed');
      setState((prev) => ({
        ...prev,
        loading: false,
        error: err,
      }));
      onError?.(err);
      throw err;
    }
  }, [endpoint, method, onSuccess, onError]);

  return {
    ...state,
    mutate,
    reset: () => {
      setState({
        data: null,
        loading: false,
        error: null,
        lastUpdated: null,
      });
    },
  };
}

// Hook for file uploads
export function useApiUpload(endpoint: string, options: UseApiOptions = {}) {
  const [state, setState] = useState<UseApiState<any>>({
    data: null,
    loading: false,
    error: null,
    lastUpdated: null,
  });

  const { onSuccess, onError } = options;

  const upload = useCallback(async (
    file: File,
    additionalData?: Record<string, any>
  ) => {
    setState((prev) => ({ ...prev, loading: true, error: null }));

    try {
      const data = await apiClient.upload(endpoint, file, additionalData);

      setState({
        data,
        loading: false,
        error: null,
        lastUpdated: new Date(),
      });

      onSuccess?.(data);
      return data;
    } catch (error) {
      const err = error instanceof Error ? error : new Error('Upload failed');
      setState((prev) => ({
        ...prev,
        loading: false,
        error: err,
      }));
      onError?.(err);
      throw err;
    }
  }, [endpoint, onSuccess, onError]);

  return {
    ...state,
    upload,
    reset: () => {
      setState({
        data: null,
        loading: false,
        error: null,
        lastUpdated: null,
      });
    },
  };
}

// Hook for authenticated API requests
export function useAuthenticatedApi<T = any>(
  endpoint: string,
  options: UseApiOptions = {}
) {
  const [state, setState] = useState<UseApiState<T>>({
    data: null,
    loading: false,
    error: null,
    lastUpdated: null,
  });

  const execute = useCallback(async (...args: any[]) => {
    setState((prev) => ({ ...prev, loading: true, error: null }));

    try {
      // Check if user is authenticated
      if (!authUtils.isAuthenticated()) {
        throw new Error('User not authenticated');
      }

      let url = endpoint;
      if (args.length > 0 && args[0]) {
        const params = new URLSearchParams();
        Object.entries(args[0]).forEach(([key, value]) => {
          if (value !== undefined && value !== null) {
            params.append(key, String(value));
          }
        });
        const queryString = params.toString();
        if (queryString) {
          url += `?${queryString}`;
        }
      }

      const data = await apiClient.get<T>(url);

      setState({
        data,
        loading: false,
        error: null,
        lastUpdated: new Date(),
      });

      options.onSuccess?.(data);
      return data;
    } catch (error) {
      const err = error instanceof Error ? error : new Error('API request failed');
      setState((prev) => ({
        ...prev,
        loading: false,
        error: err,
      }));
      options.onError?.(err);
      throw err;
    }
  }, [endpoint, options]);

  return {
    ...state,
    execute,
    reset: () => {
      setState({
        data: null,
        loading: false,
        error: null,
        lastUpdated: null,
      });
    },
  };
}

// Global loading state hook
export function useGlobalLoading() {
  const { loading, setLoading } = useUIStore();

  return {
    isLoading: loading.global || false,
    setGlobalLoading: (isLoading: boolean) => setLoading('global', isLoading),
  };
}

// API error handling hook
export function useApiError() {
  const { addNotification } = useUIStore();

  const handleError = useCallback((error: Error, context?: string) => {
    const message = error.message || 'An unexpected error occurred';

    addNotification({
      type: 'error',
      title: 'Error',
      message: context ? `${context}: ${message}` : message,
    });

    // Log error for debugging
    console.error('API Error:', error);
  }, [addNotification]);

  return { handleError };
}