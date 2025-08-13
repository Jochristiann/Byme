interface Post {
    id: string;
    description: string;
    image: string;
    category: Category;
    user: User;
    created_at: string;
}

interface Category {
    id: string;
    name: string;
}

interface NewPost{
    description: string;
}