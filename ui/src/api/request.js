import { alertStore } from '../components/AlertStack'

const API_BASE = import.meta.env.VITE_API_BASE || 'http://localhost:8000/api'

/**
 * @typedef {'get' | 'post' | 'patch' | 'delete'} Method
 */

/**
 *
 * @param {string} path
 * @param {Method} method
 * @param {string | null | undefined} auth_token
 * @param {any} body
 * @param {boolean} return_response
 * @returns
 */
const apiRequest = async (
	path,
	method,
	auth_token,
	body,
	return_response = false
) => {
	const { addAlert } = alertStore

	const fullpath = `${API_BASE}${path}`

	const headers = new Headers()
	headers.append('Content-Type', 'application/json')
	if (auth_token) {
		headers.append('Authorization', auth_token)
	}

	try {
		const response = await fetch(fullpath, {
			method,
			body: JSON.stringify(body),
			headers,
		})

		if (!response.ok) {
			throw new Error(await response.text())
		}

		if (return_response) {
			return response
		}

		try {
			return await response.json()
		} catch {}
	} catch (err) {
		addAlert(err.message, 'error')

		throw err
	}
}

/**
 *
 * @param {string} path
 * @param {string | null | undefined} auth_token
 * @param {FormData} form
 * @returns
 */
export const apiMultipartRequest = async (path, auth_token, form) => {
	const { addAlert } = alertStore

	const fullpath = `${API_BASE}${path}`

	const headers = new Headers()
	// headers.append("Content-Type", "multipart/form-data");
	if (auth_token) {
		headers.append('Authorization', auth_token)
	}

	try {
		const response = await fetch(fullpath, {
			method: 'post',
			body: form,
			headers,
		})

		if (!response.ok) {
			throw new Error(await response.text())
		}

		try {
			return await response.json()
		} catch {}
	} catch (err) {
		addAlert(err.message, 'error')

		throw err
	}
}

export default apiRequest
