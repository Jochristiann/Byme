import {create} from "zustand/react";

interface UserState {
    user: User | null;
    setUser: (user: User) => void;
    clearUser: () => void;
}

export const currentUser = create<UserState>((set) => ({
    user: null,
    setUser: (user) => set({ user }),
    clearUser: () => set({ user: null }),
}));
