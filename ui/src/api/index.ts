import api_request from "./request";

const login = async (
  username: string,
  password: string
): Promise<LoginResponse> => {
  return await api_request("/auth/login", "post", undefined, {
    username,
    password,
  });
};

const API = {
  auth: {
    login,
  },
};

export default API;
