import { onMount } from 'solid-js'
import Container from '@suid/material/Container'
import Box from '@suid/material/Box'
import TextField from '@suid/material/TextField'
import Button from '@suid/material/Button'
import Paper from '@suid/material/Paper'
import Typography from '@suid/material/Typography'
import Divider from '@suid/material/Divider'
import createLocalStore from '../../libs'
import { A, useNavigate } from '@solidjs/router'

import API from '../api'
import { alertStore } from '../components/AlertStack'

const Register = () => {
	const [store, setStore] = createLocalStore()
	const { addAlert } = alertStore
	const navigate = useNavigate()

	onMount(() => {
		if (store.access_token) {
			navigate('/')
		}
	})

	/**
	 *
	 * @param {SubmitEvent} event
	 */
	const handleSubmit = async (event) => {
		event.preventDefault()
		const data = new FormData(event.currentTarget)
		const email = data.get('email')
		const password = data.get('password')

		// Registerting
		await API.users.register(email, password)

		addAlert('You registered successfully')

		// Authenticating
		const tokenData = await API.auth.login(email, password)

		setStore('access_token', tokenData.access_token)

		const redirect_url = store.redirect || '/'
		navigate(redirect_url)
	}

	return (
		<Container maxWidth="sm" sx={{ width: 'fit-content' }}>
			<Paper sx={{ mt: '20vh' }} elevation={4}>
				<Box
					component="form"
					onSubmit={handleSubmit}
					sx={{
						px: 5,
						py: 2,
						display: 'flex',
						flexDirection: 'column',
						alignItems: 'center',
						'& > :not(style)': { my: 1.5 },
					}}
				>
					<Typography variant="h5">Registering in Pentaract</Typography>
					<Divider />
					<TextField
						name="email"
						label="email"
						type="email"
						variant="standard"
						required
					/>
					<TextField
						name="password"
						label="Password"
						variant="standard"
						type="password"
						required
					/>
					<Divider />
					<Button type="submit" variant="contained">
						Register
					</Button>

					<Divider />

					<A class="default-link" href="/login">
						Already have an account? Login!
					</A>
				</Box>
			</Paper>
		</Container>
	)
}

export default Register
