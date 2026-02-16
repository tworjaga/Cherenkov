import { useState, useEffect, useCallback } from 'react';
import { authManager, graphqlRequest } from '../lib/api';

interface User {
  id: string;
  email: string;
  name: string;
  role: 'admin' | 'operator' | 'viewer';
}

export function useAuth() {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  useEffect(() => {
    const token = authManager.getToken();
    if (token) {
      fetchUser();
    } else {
      setLoading(false);
    }

    // Listen for token changes
    const unsubscribe = authManager.onTokenChange((token) => {
      setIsAuthenticated(!!token);
      if (!token) {
        setUser(null);
      }
    });

    return unsubscribe;
  }, []);

  const fetchUser = async () => {
    try {
      const result = await graphqlRequest<{ me: User }>(
        `
        query GetCurrentUser {
          me {
            id
            email
            name
            role
          }
        }
      `,
        {},
        { useCache: false }
      );
      setUser(result.me);
      setIsAuthenticated(true);
    } catch (error) {
      authManager.clearToken();
    } finally {
      setLoading(false);
    }
  };

  const login = useCallback(async (email: string, password: string) => {
    setLoading(true);
    try {
      const result = await graphqlRequest<{ login: { token: string; refreshToken: string; user: User } }>(
        `
        mutation Login($email: String!, $password: String!) {
          login(email: $email, password: $password) {
            token
            refreshToken
            user {
              id
              email
              name
              role
            }
          }
        }
      `,
        { email, password },
        { skipAuth: true }
      );

      authManager.setToken(result.login.token, result.login.refreshToken);
      setUser(result.login.user);
      setIsAuthenticated(true);
      return true;
    } catch (error) {
      return false;
    } finally {
      setLoading(false);
    }
  }, []);

  const logout = useCallback(() => {
    authManager.clearToken();
    setUser(null);
    setIsAuthenticated(false);
  }, []);

  const hasPermission = useCallback(
    (permission: string) => {
      if (!user) return false;
      const permissions = {
        admin: ['read', 'write', 'delete', 'admin'],
        operator: ['read', 'write'],
        viewer: ['read'],
      };
      return permissions[user.role].includes(permission);
    },
    [user]
  );

  return {
    user,
    loading,
    isAuthenticated,
    login,
    logout,
    hasPermission,
  };
}
