import { alertStore } from "../components/AlertStack";

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
  const { addAlert } = alertStore;

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

    if (!response.ok) {
      throw new Error(await response.text());
    }

    return await response.json();
  } catch (err) {
    addAlert(err.message, "error");

    throw err;
  }
};

export default api_request;
