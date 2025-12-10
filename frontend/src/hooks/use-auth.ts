'use client';

import { useEffect } from 'react';
import { useAuthStore } from '@/store/auth-store';
import { authUtils } from '@/lib/auth';

export function useAuth() {
  const {
    user,
    token,
    isAuthenticated,
    isLoading,
    error,
    login,
    register,
    logout,
    // refreshToken, // TODO: Not implemented in backend
    clearError,
  } = useAuthStore();

  // Initialize auth state from localStorage on mount
  useEffect(() => {
    const storedToken = authUtils.getToken();
    const storedUser = authUtils.getUser();

    if (storedToken && storedUser && !user) {
      useAuthStore.getState().setToken(storedToken);
      useAuthStore.getState().setUser(storedUser);
    }
  }, [user]);

  // TODO: Auto-refresh token before expiration (backend doesn't have refresh endpoint yet)
  // useEffect(() => {
  //   if (!token || !isAuthenticated) return;

  //   const checkTokenExpiration = () => {
  //     if (authUtils.isTokenExpired(token)) {
  //       // If refresh fails, log out
  //       logout();
  //     }
  //   };

  //   // Check token expiration every 5 minutes
  //   const interval = setInterval(checkTokenExpiration, 5 * 60 * 1000);

  //   // Check immediately
  //   checkTokenExpiration();

  //   return () => clearInterval(interval);
  // }, [token, isAuthenticated, logout]);

  return {
    user,
    token,
    isAuthenticated,
    isLoading,
    error,
    login,
    register,
    logout,
    // refreshToken, // TODO: Not implemented in backend
    clearError,
    // Convenience methods
    isAdmin: user?.role === 'admin',
    hasRole: (role: string) => user?.role === role,
  };
}