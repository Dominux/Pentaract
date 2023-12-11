import Divider from '@suid/material/Divider'
import Box from '@suid/material/Box'
import Button from '@suid/material/Button'
import TextField from '@suid/material/TextField'
import Typography from '@suid/material/Typography'
import { createSignal } from 'solid-js'
import { useNavigate } from '@solidjs/router'
import Stack from '@suid/material/Stack'
import ChevronLeftIcon from '@suid/icons-material/ChevronLeft'

import API from '../../api'
import { alertStore } from '../../components/AlertStack'

const StorageCreateForm = () => {
	const [chatIdErr, setChatIdErr] = createSignal(null)
	const { addAlert } = alertStore
	const navigate = useNavigate()

	/**
	 *
	 * @param {SubmitEvent} event
	 */
	const handleSubmit = async (event) => {
		event.preventDefault()

		const data = new FormData(event.currentTarget)

		const name = data.get('name')
		const chatId = parseInt(data.get('chat_id'))

		await API.storages.createStorage(name, chatId)

		addAlert(`Created storage "${name}"`, 'success')

		navigate('/storages')
	}

	/**
	 *
	 * @param {SubmitEvent} event
	 */
	const validateChatId = (event) => {
		event.preventDefault()
		const value = event.currentTarget.value

		let err = null

		if (value > 0) {
			err = 'Chat id must be a valid negative integer'
		} else if (value === '') {
			err = 'Chat id is required and must be a valid negative integer'
		}

		setChatIdErr(err)
	}

	return (
		<Stack sx={{ maxWidth: 540, minWidth: 320, mx: 'auto' }}>
			<Box>
				<Button
					onClick={() => navigate('/storages')}
					variant="outlined"
					startIcon={<ChevronLeftIcon />}
				>
					Back
				</Button>
			</Box>

			<Box
				component="form"
				onSubmit={handleSubmit}
				sx={{
					py: 2,
					mx: 'auto',
					maxWidth: 400,
					display: 'flex',
					flexDirection: 'column',
					alignItems: 'center',
					'& > :not(style)': { my: 1.5 },
				}}
			>
				<Typography variant="h5">Register new storage</Typography>
				<Divider />
				<TextField
					id="name"
					name="name"
					label="Name"
					variant="standard"
					fullWidth
					required
				/>
				<TextField
					id="chat_id"
					name="chat_id"
					label="Chat id"
					type="number"
					variant="standard"
					onChange={validateChatId}
					helperText={chatIdErr}
					error={typeof chatIdErr() === 'string'}
					fullWidth
					required
				/>
				<Button type="submit" variant="contained" color="success">
					Register
				</Button>
			</Box>
		</Stack>
	)
}

export default StorageCreateForm
