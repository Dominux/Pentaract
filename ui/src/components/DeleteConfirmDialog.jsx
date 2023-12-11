import Button from '@suid/material/Button'
import Dialog from '@suid/material/Dialog'
import DialogActions from '@suid/material/DialogActions'
import DialogContent from '@suid/material/DialogContent'
import DialogTitle from '@suid/material/DialogTitle'
import DialogContentText from '@suid/material/DialogContentText'

/**
 * @typedef {Object} DeleteConfirmDialogProps
 * @property {boolean} isOpened
 * @property {string} entity
 * @property {string} entityId
 * @property {() => void} onConfirm
 * @property {() => void} onCancel
 */

/**
 *
 * @param {DeleteConfirmDialogProps} props
 */
const DeleteConfirmDialog = (props) => {
	return (
		<Dialog open={props.isOpened} onClose={props.onCancel}>
			<DialogTitle>Delete {props.entity}?</DialogTitle>
			<DialogContent>
				<DialogContentText>
					Are you sure you want to delete {props.entity} {props.entityId}?
				</DialogContentText>
			</DialogContent>

			<DialogActions>
				<Button onClick={props.onConfirm} color="warning">
					Delete
				</Button>
				<Button onClick={props.onCancel} color="info">
					Cancel
				</Button>
			</DialogActions>
		</Dialog>
	)
}

export default DeleteConfirmDialog
