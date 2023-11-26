type Method = "get" | "post" | "patch" | "delete";

const API_BASE = "http://localhost:8080/api";

const api_request = async <T>(
  path: string,
  method: Method,
  auth_token?: string,
  body?: any
): Promise<T> => {
  const fullpath = `${API_BASE}${path}`;

  const headers = new Headers();
  headers.append("Content-Type", "application/json");
  if (auth_token) {
    headers.append("Authorization", auth_token);
  }

  try {
    const response = await fetch(fullpath, {
      method,
      body: JSON.stringify(body),
      headers,
    });
    return await response.json();
  } catch (error) {
    // TODO: add graceful error handling
    throw error;
  }
};

export default api_request;
