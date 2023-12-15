import Button from '@suid/material/Button'
import TextField from '@suid/material/TextField'
import Dialog from '@suid/material/Dialog'
import DialogActions from '@suid/material/DialogActions'
import DialogContent from '@suid/material/DialogContent'
import DialogTitle from '@suid/material/DialogTitle'
import Select from '@suid/material/Select'
import MenuItem from '@suid/material/MenuItem'
import FormControl from '@suid/material/FormControl'
import InputLabel from '@suid/material/InputLabel'
import { useParams } from '@solidjs/router'

import AccessTypeChip, { makeAccessTypeUserFriendly } from './AccessTypeChip'
import API from '../api'
import { alertStore } from './AlertStack'

/**
 * @typedef {Object} GrantAccessProps
 * @property {boolean} isVisible
 * @property {() => void} onClose
 * @property {() => void} afterGrant
 * @property {string | undefined} email
 */

/**
 *
 * @param {GrantAccessProps} props
 */
const GrantAccess = (props) => {
	const { addAlert } = alertStore
	const params = useParams()
	const getAction = () => (props.email?.length ? 'Change' : 'Grant')

	/**
	 *
	 * @param {SubmitEvent} event
	 */
	const onGrant = async (event) => {
		event.preventDefault()

		const data = new FormData(event.currentTarget)
		const email = props.email || data.get('email')
		const access_type = data.get('access_type')

		await API.access.grantAccess(params.id, email, access_type)

		props.onClose()
		addAlert(
			`Granted "${makeAccessTypeUserFriendly(
				access_type
			)}" access to the user with email "${email}"`,
			'success'
		)

		props.afterGrant()
	}

	return (
		<>
			<Dialog open={props.isVisible} onClose={props.onClose}>
				<form onSubmit={onGrant}>
					<DialogTitle>{getAction} access</DialogTitle>
					<DialogContent>
						<TextField
							required
							defaultValue={props.email}
							disabled={props.email}
							margin="normal"
							id="email"
							label="User's email"
							type="email"
							name="email"
							fullWidth
							variant="standard"
						/>

						<FormControl fullWidth>
							<InputLabel id="email-select-label">Access Type</InputLabel>
							<Select
								variant="standard"
								labelId="email-select-label"
								label="Access Type"
								name="access_type"
							>
								{['R', 'W', 'A'].map((at) => (
									<MenuItem value={at}>
										<AccessTypeChip at={at} />
									</MenuItem>
								))}
							</Select>
						</FormControl>
					</DialogContent>
					<DialogActions>
						<Button type="submit" color="success">
							{getAction}
						</Button>

						<Button onClick={props.onClose} color="error">
							Cancel
						</Button>
					</DialogActions>
				</form>
			</Dialog>
		</>
	)
}

export default GrantAccess
