import axios from "axios";

const baseURL = "http://127.0.0.";

export const apiAuth = axios.create({
    baseURL: baseURL+"1:3000",
    timeout: 10000,
});

export const apiUser = axios.create({
    baseURL: baseURL+"2:3000",
    timeout: 10000,
});

export const apiPost = axios.create({
    baseURL: baseURL+"3:3000",
    timeout: 10000,
});

export const apiComment = axios.create({
    baseURL: baseURL+"4:3000",
    timeout: 10000,
});