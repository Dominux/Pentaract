const units = ['bytes', 'KiB', 'MiB', 'GiB', 'TiB']

/**
 *
 * @param {number} size
 * @returns {string}
 */
export const convertSize = (size) => {
	let l = 0,
		n = size

	while (n >= 1024 && l < units.length - 1 && ++l) {
		n = n / 1024
	}

	return `${n.toFixed(n < 10 && l > 0 ? 1 : 0)} ${units[l]}`
}
