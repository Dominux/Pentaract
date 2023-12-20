import Divider from '@suid/material/Divider'
import Box from '@suid/material/Box'
import Button from '@suid/material/Button'
import TextField from '@suid/material/TextField'
import Select from '@suid/material/Select'
import InputLabel from '@suid/material/InputLabel'
import FormControl from '@suid/material/FormControl'
import Typography from '@suid/material/Typography'
import { createSignal, mapArray, onMount } from 'solid-js'
import { useNavigate } from '@solidjs/router'
import Stack from '@suid/material/Stack'
import MenuItem from '@suid/material/MenuItem'
import IconButton from '@suid/material/IconButton'
import HelpOutlineIcon from '@suid/icons-material/HelpOutline'
import ChevronLeftIcon from '@suid/icons-material/ChevronLeft'

import API from '../../api'
import { alertStore } from '../../components/AlertStack'

const StorageWorkerCreateForm = () => {
	/**
	 * @type {[import("solid-js").Accessor<import("../../api").StorageWorker[]>, any]}
	 */
	const [storages, setStorages] = createSignal([])
	const { addAlert } = alertStore
	const navigate = useNavigate()

	onMount(async () => {
		const storagesSchema = await API.storages.listStorages()
		setStorages(storagesSchema.storages)
	})

	/**
	 *
	 * @param {SubmitEvent} event
	 */
	const handleSubmit = async (event) => {
		event.preventDefault()

		const data = new FormData(event.currentTarget)

		const name = data.get('name')
		const token = data.get('token')
		const storageId = data.get('storage_id')

		await API.storageWorkers.createStorageWorker(name, token, storageId)

		addAlert(`Created storage worker "${name}"`, 'success')

		navigate('/storage_workers')
	}

	return (
		<Stack sx={{ maxWidth: 540, minWidth: 320, mx: 'auto' }}>
			<Box>
				<Button
					onClick={() => navigate('/storage_workers')}
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
				<Typography variant="h5">
					Register new storage worker
					<a
						href="https://github.com/Dominux/Pentaract/wiki/Creating-storage-workers"
						target="_blank"
					>
						<IconButton color="warning" sx={{ py: 0 }}>
							<HelpOutlineIcon />
						</IconButton>
					</a>
				</Typography>
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
					id="token"
					name="token"
					label="Token"
					variant="standard"
					fullWidth
					required
				/>

				<FormControl fullWidth variant="standard" required>
					<InputLabel id="storage-select-label">Storage</InputLabel>
					<Select
						labelId="storage-select-label"
						label="Storage"
						name="storage_id"
					>
						{mapArray(storages, (storage) => (
							<MenuItem value={storage.id}>{storage.name}</MenuItem>
						))}
					</Select>
				</FormControl>

				<Button type="submit" variant="contained" color="secondary">
					Register
				</Button>
			</Box>
		</Stack>
	)
}

export default StorageWorkerCreateForm
