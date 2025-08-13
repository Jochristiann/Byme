interface User{
    Name: string;
    Username: string;
    Email: string;
    Dob: string|null;
    Image: string|null;
    Role: string;
    origin: Origin;
    created_at: string;
    updated_at: string;
    token: string|null;
}

interface Origin{
    Name: string;
}

interface NewUser{
    Username: string;
    Email: string;
    Password: string;
}