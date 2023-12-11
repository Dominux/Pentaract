import Alert from '@suid/material/Alert'
import Stack from '@suid/material/Stack'
import { For, createRoot, createSignal } from 'solid-js'

/**
 * @typedef {"error" | "warning" | "info" | "success"} AlertSeverity
 */

/**
 * @typedef {Object} AlertType
 * @property {string} msg
 * @property {AlertSeverity} severity
 */

export const alertStore = createRoot(() => {
	/**
	 * @type {[import("solid-js").Accessor<AlertType[]>, import("solid-js").Setter<AlertType[]>]}
	 */
	const [alertList, setAlertList] = createSignal([])

	/**
	 *
	 * @param {string} msg
	 * @param {AlertSeverity} severity
	 * @returns
	 */
	const addAlert = (msg, severity) => {
		setAlertList((alertList) => [{ msg, severity }, ...alertList])

		setTimeout(() => setAlertList((alertList) => alertList.slice(0, -1)), 5e3)
	}

	return { alertList, addAlert }
})

const AlertStack = () => {
	const { alertList } = alertStore

	return (
		<Stack
			sx={{
				position: 'fixed',
				zIndex: 99999,
				right: '1rem',
				top: '5rem',
				maxWidth: 360,
				width: '30vw',
				minWidth: 240,
			}}
			spacing={1}
		>
			<For each={alertList()}>
				{(alert) => <Alert severity={alert.severity}>{alert.msg}</Alert>}
			</For>
		</Stack>
	)
}

export default AlertStack
