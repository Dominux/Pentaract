import createLocalStore from "../../libs";

import apiRequest from "./request";

/**
 * @typedef {Object} TokenData
 * @property {string} access_token
 */

/**
 *
 * @param {string} username
 * @param {string} password
 * @returns {Promise<TokenData>}
 */
const login = async (username, password) => {
  return await apiRequest("/auth/login", "post", undefined, {
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
  return await apiRequest("/storages", "post", getAuthToken(), {
    name,
    chat_id,
  });
};

/**
 * @typedef {Object} Storage
 * @property {string} id
 * @property {string} name
 * @property {number} chat_id
 */

/**
 * @typedef {Object} StoragesSchema
 * @property {Storage[]} storages
 */

/**
 *
 * @returns {Promise<StoragesSchema>}
 */
const listStorages = async () => {
  return await apiRequest("/storages", "get", getAuthToken());
};

/**
 * @typedef {Object} StorageWorker
 * @property {string} id
 * @property {string} name
 * @property {number} storage_id
 * @property {number} token
 */

/**
 *
 * @param {string} name
 * @param {string} token
 * @param {string | null | undefined} storage_id
 * @returns {Promise<StorageWorker>}
 */
const createStorageWorker = async (name, token, storage_id) => {
  return await apiRequest("/storage_workers", "post", getAuthToken(), {
    name,
    token,
    storage_id,
  });
};

/**
 *
 * @returns {Promise<StorageWorker[]>}
 */
const listStorageWorkers = async () => {
  return await apiRequest("/storage_workers", "get", getAuthToken());
};

const API = {
  auth: {
    login,
  },
  storages: {
    createStorage,
    listStorages,
  },
  storageWorkers: {
    createStorageWorker,
    listStorageWorkers,
  },
};

const getAuthToken = () => {
  const [store, _setStore] = createLocalStore();
  return `Bearer ${store.access_token}`;
};

export default API;
