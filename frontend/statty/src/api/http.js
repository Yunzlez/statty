import Axios from "axios";

const axiosInstance = Axios.create({
    baseURL: import.meta.env.VITE_APP_BACKEND_URL + import.meta.env.VITE_APP_API_PATH
});

export const Client = {
    get(url) {
        return axiosInstance.get(url);
    },
    post(url, body) {
        return axiosInstance.post(url, body);
    },
    put(url, body) {
        return axiosInstance.put(url, body);
    },
    delete(url) {
        return axiosInstance.delete(url);
    }
};