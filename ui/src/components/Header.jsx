import AppBar from '@suid/material/AppBar'
import Toolbar from '@suid/material/Toolbar'
import Typography from '@suid/material/Typography'
import IconButton from '@suid/material/IconButton'
import { A, useNavigate } from '@solidjs/router'
import createLocalStore from '../../libs'
import LogoutIcon from '@suid/icons-material/Logout'

const Header = () => {
	const [_store, setStore] = createLocalStore()
	const navigate = useNavigate()

	const logout = (_) => {
		setStore('access_token')
		setStore('redirect', '/')

		navigate('/login')
	}

	return (
		<AppBar>
			<Toolbar sx={{ justifyContent: 'space-between' }}>
				<Typography variant="h4" noWrap component="div">
					<A href="/">Pentaract</A>
				</Typography>

				<IconButton onClick={logout}>
					<LogoutIcon sx={{ color: 'white' }} />
				</IconButton>
			</Toolbar>
		</AppBar>
	)
}

export default Header
