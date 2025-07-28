import axios from "axios";

export const apiAuth = axios.create({
    baseURL: "http://localhost:3000",
    timeout: 10000,
});

export const apiUser = axios.create({
    baseURL: "http://localhost:3001",
    timeout: 10000,
});

