import { redirect } from '@tanstack/react-router';
import { Route as LoginRoute } from "./routes/auth/login";
import * as React from "react";
import { axiosClient, isAdmin as isAdminJwt } from '@starfoundry/components/services/client';

export interface AuthContext {
    isAuthenticated: () => Promise<boolean>;
    isAdmin: () => Promise<boolean>;
    login: () => any; // type signature is from redirect()
}

const AuthContext = React.createContext<AuthContext | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
    const login = React.useCallback(() => {
        return redirect({
            to: LoginRoute.fullPath,
        });
    }, []);

    const isAuthenticated = React.useCallback(async () => {
        return axiosClient()
            .then(_ => {
                return true;
            })
            .catch(_ => {
                return false;
            })
    }, []);

    const isAdmin = React.useCallback(async () => {
        return isAdminJwt()
            .catch(_ => false);
    }, []);

    return (
        <AuthContext.Provider value={{ isAuthenticated, isAdmin, login }}>
            {children}
        </AuthContext.Provider>
    );
}

export function useAuth() {
    const context = React.useContext(AuthContext);
    if (!context) {
        throw new Error("useAuth must be used within an AuthProvider");
    }
    return context;
}
