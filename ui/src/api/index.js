import api_request from "./request";

/**
 * @typedef {Object} TokenData
 * @property {string} access_token
 */

/**
 *
 * @param {string} username
 * @param {string} password
 * @returns {TokenData}
 */
const login = async (username, password) => {
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
