import * as React from "react";
import { axiosClient } from "./services/client";

import { redirect } from '@tanstack/react-router';
import { Route as LoginRoute } from "./routes/auth/login";

export interface AuthContext {
    isAuthenticated: () => Promise<boolean>;
    login: () => Promise<any>; // type signature is from redirect()
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

    return (
        <AuthContext.Provider value={{ isAuthenticated, login }}>
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
