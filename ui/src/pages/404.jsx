import Typography from '@suid/material/Typography'
import Box from '@suid/material/Box'

const NotFound = () => {
	return (
		<Box
			sx={{
				display: 'flex',
				alignItems: 'center',
				justifyContent: 'center',
				flexDirection: 'column',
				mt: 16,
			}}
		>
			<Typography variant="h1">404</Typography>
			<Typography variant="h4">Not found</Typography>
		</Box>
	)
}

export default NotFound
