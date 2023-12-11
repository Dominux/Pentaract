/**
   My modification of some package:
   https://www.npmjs.com/package/@solid-primitives/local-store
 */

import { createSignal, getListener } from 'solid-js'
/**
 * Create a new storage primitive that can retain any data type
 * with an interface compatible with the Web Storage API.
 *
 * @param prefix - Prefix to wrap all stored values with.
 * @param storage - Storage engine to use for recording the value
 * @return Returns a state reader, setter and clear function
 *
 * @example
 * ```ts
 * const [value, setValue] = createStorage('app');
 * setValue('My new value');
 * console.log(value());
 * ```
 */
function createLocalStore(prefix = null, storage = localStorage) {
	const signals = new Map()
	const propPrefix = prefix === null ? '' : `${prefix}.`
	return [
		new Proxy(
			{},
			{
				get(_, key) {
					if (key === 'toJSON') {
						return storage.getAll ? () => storage.getAll() : () => storage
					}
					if (getListener()) {
						let node = signals.get(key)
						if (!node) {
							node = createSignal(undefined, { equals: false })
							signals.set(key, node)
						}
						node[0]()
					}
					const value = storage.getItem(`${propPrefix}${key}`)
					try {
						return JSON.parse(value)
					} catch (_) {
						return undefined
					}
				},
			}
		),
		(key, value) => {
			storage.setItem(`${propPrefix}${key}`, JSON.stringify(value))
			const node = signals.get(key)
			node && node[1]()
		},
		(key) => {
			storage.removeItem(`${propPrefix}${key}`)
			const node = signals.get(key)
			node && node[1]()
		},
		() => {
			storage.clear()
			signals.clear()
		},
	]
}
export default createLocalStore
