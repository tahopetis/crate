import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { UIState, Notification } from '@/lib/types';

interface UIStore extends UIState {
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  setTheme: (theme: 'light' | 'dark' | 'system') => void;
  setLoading: (key: string, loading: boolean) => void;
  setError: (key: string, error: string | null) => void;
  clearErrors: () => void;
  addNotification: (notification: Omit<Notification, 'id' | 'timestamp' | 'read'>) => void;
  removeNotification: (id: string) => void;
  markNotificationRead: (id: string) => void;
  clearNotifications: () => void;
  setGlobalLoading: (loading: boolean) => void;
}

export const useUIStore = create<UIStore>()(
  persist(
    (set, get) => ({
      // Initial state
      sidebarOpen: true,
      theme: 'system',
      loading: {},
      errors: {},
      notifications: [],

      // Sidebar actions
      toggleSidebar: () => {
        set((state) => ({ sidebarOpen: !state.sidebarOpen }));
      },

      setSidebarOpen: (open: boolean) => {
        set({ sidebarOpen: open });
      },

      // Theme actions
      setTheme: (theme: 'light' | 'dark' | 'system') => {
        set({ theme });
        // Apply theme to document
        const root = document.documentElement;
        if (theme === 'dark') {
          root.classList.add('dark');
        } else if (theme === 'light') {
          root.classList.remove('dark');
        } else {
          // System preference
          if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
            root.classList.add('dark');
          } else {
            root.classList.remove('dark');
          }
        }
      },

      // Loading state actions
      setLoading: (key: string, loading: boolean) => {
        set((state) => ({
          loading: {
            ...state.loading,
            [key]: loading,
          },
        }));
      },

      setGlobalLoading: (loading: boolean) => {
        set((state) => ({
          loading: {
            ...state.loading,
            global: loading,
          },
        }));
      },

      // Error state actions
      setError: (key: string, error: string | null) => {
        set((state) => ({
          errors: {
            ...state.errors,
            [key]: error,
          },
        }));
      },

      clearErrors: () => {
        set({ errors: {} });
      },

      // Notification actions
      addNotification: (notification: Omit<Notification, 'id' | 'timestamp' | 'read'>) => {
        const newNotification: Notification = {
          ...notification,
          id: Math.random().toString(36).substr(2, 9),
          timestamp: new Date().toISOString(),
          read: false,
        };

        set((state) => ({
          notifications: [newNotification, ...state.notifications].slice(0, 50), // Keep only last 50
        }));
      },

      removeNotification: (id: string) => {
        set((state) => ({
          notifications: state.notifications.filter((n) => n.id !== id),
        }));
      },

      markNotificationRead: (id: string) => {
        set((state) => ({
          notifications: state.notifications.map((n) =>
            n.id === id ? { ...n, read: true } : n
          ),
        }));
      },

      clearNotifications: () => {
        set({ notifications: [] });
      },
    }),
    {
      name: 'ui-storage',
      partialize: (state) => ({
        sidebarOpen: state.sidebarOpen,
        theme: state.theme,
      }),
    }
  )
);

// Initialize theme on app start
if (typeof window !== 'undefined') {
  const { theme } = useUIStore.getState();
  const root = document.documentElement;

  if (theme === 'dark') {
    root.classList.add('dark');
  } else if (theme === 'light') {
    root.classList.remove('dark');
  } else {
    // System preference
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }
  }
}