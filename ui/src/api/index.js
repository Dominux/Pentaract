import createLocalStore from "../../libs";

import apiRequest, { apiMultipartRequest } from "./request";

/////////////////////////////////////////////////////////////
////  AUTH
/////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////////////////////
////  STORAGES
/////////////////////////////////////////////////////////////

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
 * @param {string} id
 * @returns {Promise<Storage>}
 */
const getStorage = async (id) => {
  return await apiRequest(`/storages/${id}`, "get", getAuthToken());
};

/////////////////////////////////////////////////////////////
////  STORAGE WORKERS
/////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////////////////////
////  FILES
/////////////////////////////////////////////////////////////

/**
 *
 * @param {string} storage_id
 * @param {string} path
 * @returns
 */
const createFolder = async (storage_id, path) => {
  return await apiRequest(
    `/storages/${storage_id}/files/create_folder`,
    "get",
    { path }
  );
};

/**
 *
 * @param {string} storage_id
 * @param {string} path
 * @param {string} file
 * @returns
 */
const uploadFile = async (storage_id, path, file) => {
  const form = new FormData();
  form.append("file", file);
  form.append("path", path);

  return await apiMultipartRequest(
    `/storages/${storage_id}/files/upload`,
    getAuthToken(),
    form
  );
};

/**
 *
 * @param {string} storage_id
 * @param {string} path
 * @param {string} file
 * @returns
 */
const uploadFileTo = async (storage_id, path, file) => {
  const form = new FormData();
  form.append("file", file);
  form.append("path", path);

  return await apiMultipartRequest(
    `/storages/${storage_id}/files/upload_to`,
    getAuthToken(),
    form
  );
};

/**
 * @typedef {Object} FSElement
 * @property {string} path
 * @property {string} name
 * @property {boolean} is_file
 */

/**
 *
 * @param {string} storage_id
 * @param {string} path
 * @returns {Promise<FSElement[]>}
 */
const getFSLayer = async (storage_id, path) => {
  return await apiRequest(
    `/storages/${storage_id}/files/tree/${path}`,
    "get",
    getAuthToken()
  );
};

/////////////////////////////////////////////////////////////
////  API
/////////////////////////////////////////////////////////////

const API = {
  auth: {
    login,
  },
  storages: {
    createStorage,
    listStorages,
    getStorage,
  },
  storageWorkers: {
    createStorageWorker,
    listStorageWorkers,
  },
  files: {
    createFolder,
    uploadFile,
    uploadFileTo,
    getFSLayer,
  },
};

const getAuthToken = () => {
  const [store, _setStore] = createLocalStore();
  return `Bearer ${store.access_token}`;
};

export default API;
