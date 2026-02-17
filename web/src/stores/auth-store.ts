import { create } from 'zustand';

interface AuthState {
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  user: { id: string; email: string; name: string } | null;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
  clearError: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  isAuthenticated: false,
  isLoading: false,
  error: null,
  user: null,

  login: async (email: string, password: string) => {
    set({ isLoading: true, error: null });
    
    try {
      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 1000));
      
      if (email && password) {
        set({
          isAuthenticated: true,
          user: {
            id: '1',
            email,
            name: 'Admin User',
          },
          isLoading: false,
        });
      } else {
        set({
          error: 'Invalid credentials',
          isLoading: false,
        });
      }
    } catch {
      set({
        error: 'Login failed',
        isLoading: false,
      });
    }
  },

  logout: () => {
    set({
      isAuthenticated: false,
      user: null,
      error: null,
    });
  },

  clearError: () => {
    set({ error: null });
  },
}));
