import AppBar from '@suid/material/AppBar'
import Toolbar from '@suid/material/Toolbar'
import Typography from '@suid/material/Typography'
import IconButton from '@suid/material/IconButton'
import { A, useNavigate } from '@solidjs/router'
import LogoutIcon from '@suid/icons-material/Logout'
import Box from '@suid/material/Box'

import AppIcon from './AppIcon'
import createLocalStore from '../../libs'

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
				<A href="/">
					<Box sx={{ display: 'flex', alignItems: 'center' }}>
						<AppIcon />
						<Typography variant="h4" noWrap sx={{ pl: 1.5 }}>
							Pentaract
						</Typography>
					</Box>
				</A>

				<IconButton onClick={logout}>
					<LogoutIcon sx={{ color: 'white' }} />
				</IconButton>
			</Toolbar>
		</AppBar>
	)
}

export default Header
