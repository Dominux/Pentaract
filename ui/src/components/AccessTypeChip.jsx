import { ThemeProvider, createTheme } from '@suid/material'
import red from '@suid/material/colors/red'
import amber from '@suid/material/colors/amber'
import lightBlue from '@suid/material/colors/lightBlue'
import Chip from '@suid/material/Chip'

export const makeAccessTypeUserFriendly = (at) => {
	switch (at) {
		case 'A':
			return 'Admin'
		case 'W':
			return 'Can edit'
		case 'R':
			return 'Viewer'
	}
}

const accessTypeTheme = createTheme({
	palette: {
		A: {
			main: red[500],
			light: red[100],
			dark: red[400],
			contrastText: '#fff',
		},
		W: {
			main: amber[500],
			light: amber[100],
			dark: amber[400],
			contrastText: '#fff',
		},
		R: {
			main: lightBlue[500],
			light: lightBlue[100],
			dark: lightBlue[200],
			contrastText: '#fff',
		},
	},
})

/**
 * @typedef {Object} AccessTypeChipProps
 * @property {import('../api').AccessType} at
 */

/**
 *
 * @param {AccessTypeChipProps} props
 */
const AccessTypeChip = (props) => {
	return (
		<ThemeProvider theme={accessTypeTheme}>
			<Chip label={makeAccessTypeUserFriendly(props.at)} color={props.at} />
		</ThemeProvider>
	)
}

export default AccessTypeChip
