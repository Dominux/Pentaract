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

const Login = () => {
	const [store, setStore] = createLocalStore()
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

		const tokenData = await API.auth.login(email, password)

		setStore('access_token', tokenData.access_token)
		setStore('user', { email })

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
					<Typography variant="h5">Pentaract Account</Typography>
					<Divider />
					<TextField
						name="email"
						label="email"
						variant="standard"
						type="email"
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
						Login
					</Button>

					<Divider />

					<A class="default-link" href="/register">
						Don't have an account yet? Register!
					</A>
				</Box>
			</Paper>
		</Container>
	)
}

export default Login
