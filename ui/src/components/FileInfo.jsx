import Grid from '@suid/material/Grid'
import Dialog from '@suid/material/Dialog'
import DialogContent from '@suid/material/DialogContent'
import DialogTitle from '@suid/material/DialogTitle'
import Typography from '@suid/material/Typography'

import { convertSize } from '../common/size_converter'

/**
 * @typedef {Object} FileInfoDialogProps
 * @property {import('../api').FSElement} file
 * @property {boolean} isOpened
 * @property {() => void} onClose
 */

/**
 *
 * @param {FileInfoDialogProps} props
 * @returns
 */
const FileInfoDialog = (props) => {
	return (
		<>
			<Dialog open={props.isOpened} onClose={props.onClose}>
				<DialogTitle sx={{ textAlign: 'center' }}>File info</DialogTitle>
				<DialogContent>
					<Grid container spacing={4}>
						<Grid item>
							<Typography fontStyle="italic">Size</Typography>
						</Grid>
						<Grid item>{convertSize(props.file.size)}</Grid>
					</Grid>
				</DialogContent>
			</Dialog>
		</>
	)
}

export default FileInfoDialog
