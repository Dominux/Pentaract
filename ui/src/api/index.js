import createLocalStore from '../../libs'

import apiRequest, { apiMultipartRequest } from './request'

/////////////////////////////////////////////////////////////
////  USERS
/////////////////////////////////////////////////////////////

/**
 * @typedef {Object} TokenData
 * @property {string} access_token
 */

/**
 *
 * @param {string} email
 * @param {string} password
 * @returns {Promise<any>}
 */
const register = async (email, password) => {
	return await apiRequest('/users', 'post', undefined, {
		email,
		password,
	})
}

/////////////////////////////////////////////////////////////
////  AUTH
/////////////////////////////////////////////////////////////

/**
 * @typedef {Object} TokenData
 * @property {string} access_token
 */

/**
 *
 * @param {string} email
 * @param {string} password
 * @returns {Promise<TokenData>}
 */
const login = async (email, password) => {
	return await apiRequest('/auth/login', 'post', undefined, {
		email,
		password,
	})
}

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
	return await apiRequest('/storages', 'post', getAuthToken(), {
		name,
		chat_id,
	})
}

/**
 * @typedef {Object} Storage
 * @property {string} id
 * @property {string} name
 * @property {number} chat_id
 */

/**
 * @typedef {Object} StorageWithInfoProperties
 * @property {number} size
 * @property {number} files_amount
 * @typedef {Storage & StorageWithInfoProperties} StorageWithInfo
 */

/**
 * @typedef {Object} StoragesSchema
 * @property {StorageWithInfo[]} storages
 */

/**
 *
 * @returns {Promise<StoragesSchema>}
 */
const listStorages = async () => {
	return await apiRequest('/storages', 'get', getAuthToken())
}

/**
 * @param {string} id
 * @returns {Promise<Storage>}
 */
const getStorage = async (id) => {
	return await apiRequest(`/storages/${id}`, 'get', getAuthToken())
}

/////////////////////////////////////////////////////////////
////  ACCESS
/////////////////////////////////////////////////////////////

/**
 * @typedef {'R' | 'W' | 'A'} AccessType
 */

/**
 * @typedef {Object} UserWithAccess
 * @property {string} id
 * @property {string} email
 * @property {AccessType} access_type
 */

/**
 *
 * @param {string} storageID
 * @param {string} email
 * @param {AccessType} accessType
 * @returns
 */
const grantAccess = async (storageID, email, accessType) => {
	return await apiRequest(
		`/storages/${storageID}/access`,
		'post',
		getAuthToken(),
		{ user_email: email, access_type: accessType }
	)
}

/**
 *
 * @param {string} storageID
 * @returns {Promise<UserWithAccess[]>}
 */
const listUsersWithAccess = async (storageID) => {
	return await apiRequest(
		`/storages/${storageID}/access`,
		'get',
		getAuthToken()
	)
}

/**
 *
 * @param {string} storageID
 * @param {string} userID
 * @returns
 */
const restrictAccess = async (storageID, userID) => {
	return await apiRequest(
		`/storages/${storageID}/access`,
		'delete',
		getAuthToken(),
		{ user_id: userID }
	)
}

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
	return await apiRequest('/storage_workers', 'post', getAuthToken(), {
		name,
		token,
		storage_id,
	})
}

/**
 *
 * @returns {Promise<StorageWorker[]>}
 */
const listStorageWorkers = async () => {
	return await apiRequest('/storage_workers', 'get', getAuthToken())
}

/////////////////////////////////////////////////////////////
////  FILES
/////////////////////////////////////////////////////////////

/**
 *
 * @param {string} storage_id
 * @param {string} path
 * @param {string} folderName
 * @returns
 */
const createFolder = async (storage_id, path, folderName) => {
	return await apiRequest(
		`/storages/${storage_id}/files/create_folder`,
		'post',
		getAuthToken(),
		{ path, folder_name: folderName }
	)
}

/**
 *
 * @param {string} storage_id
 * @param {string} path
 * @param {any} file
 * @returns
 */
const uploadFile = async (storage_id, path, file) => {
	const form = new FormData()
	form.append('file', file)
	form.append('path', path)

	return await apiMultipartRequest(
		`/storages/${storage_id}/files/upload`,
		getAuthToken(),
		form
	)
}

/**
 *
 * @param {string} storage_id
 * @param {string} path
 * @param {any} file
 * @returns
 */
const uploadFileTo = async (storage_id, path, file) => {
	const form = new FormData()
	form.append('file', file)
	form.append('path', path)

	return await apiMultipartRequest(
		`/storages/${storage_id}/files/upload_to`,
		getAuthToken(),
		form
	)
}

/**
 * @typedef {Object} FSElement
 * @property {string} path
 * @property {string} name
 * @property {boolean} is_file
 * @property {number} size
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
		'get',
		getAuthToken()
	)
}

/**
 *
 * @param {string} storage_id
 * @param {string} path
 * @returns {Promise<Blob>}
 */
const download = async (storage_id, path) => {
	const response = await apiRequest(
		`/storages/${storage_id}/files/download/${path}`,
		'get',
		getAuthToken(),
		undefined,
		true
	)

	return await response.blob()
}

/**
 *
 * @param {string} storage_id
 * @param {string} path
 */
const deleteFile = async (storage_id, path) => {
	await apiRequest(
		`/storages/${storage_id}/files/${path}`,
		'delete',
		getAuthToken()
	)
}

/////////////////////////////////////////////////////////////
////  API
/////////////////////////////////////////////////////////////

const API = {
	users: {
		register,
	},
	auth: {
		login,
	},
	storages: {
		createStorage,
		listStorages,
		getStorage,
	},
	access: {
		grantAccess,
		listUsersWithAccess,
		restrictAccess,
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
		download,
		deleteFile,
	},
}

const getAuthToken = () => {
	const [store, _setStore] = createLocalStore()
	return `Bearer ${store.access_token}`
}

export default API
