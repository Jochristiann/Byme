interface User{
    name: string;
    gender: string;
    username: string;
    email: string;
    dob: string|null;
    bio: string;
    image: string|null;
    role: string;
    isverified: boolean;
    origin: Origin|null;
    created_at: string;
    updated_at: string;
    token: string|null;
}

interface Origin{
    name: string;
}

interface NewUser{
    username: string;
    umail: string;
    password: string;
}