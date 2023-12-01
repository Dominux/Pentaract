import createLocalStore from "../../libs";

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

/**
 *
 * @param {string} name
 * @param {number} chat_id
 * @returns
 */
const createStorage = async (name, chat_id) => {
  return await api_request("/storages", "post", getAuthToken(), {
    name,
    chat_id,
  });
};

const API = {
  auth: {
    login,
  },
  storages: {
    createStorage,
  },
};

const getAuthToken = () => {
  const [store, _setStore] = createLocalStore();
  return `Bearer ${store.access_token}`;
};

export default API;
