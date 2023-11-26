const API_BASE = "http://localhost:8080/api";

/**
 * @typedef {'get' | 'post' | 'patch' | 'delete'} Method
 */

/**
 *
 * @param {string} path
 * @param {Method} method
 * @param {string | null | undefined} auth_token
 * @param {any} body
 * @returns
 */
const api_request = async (path, method, auth_token, body) => {
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
