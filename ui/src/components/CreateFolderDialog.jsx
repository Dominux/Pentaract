import Button from '@suid/material/Button'
import TextField from '@suid/material/TextField'
import Dialog from '@suid/material/Dialog'
import DialogActions from '@suid/material/DialogActions'
import DialogContent from '@suid/material/DialogContent'
import DialogTitle from '@suid/material/DialogTitle'
import { createEffect, createSignal } from 'solid-js'

/**
 * @typedef {Object} CreateFolderDialogProps
 * @property {boolean} isOpened
 * @property {(folderName: string) => Promise<void>} onCreate
 * @property {() => void} onClose
 */

/**
 *
 * @param {CreateFolderDialogProps} props
 * @returns
 */
const CreateFolderDialog = (props) => {
	const [errFolderName, setErrFolderName] = createSignal(null)
	const [folderName, setFolderName] = createSignal('')

	let folderNameElement

	createEffect(() => {
		if (props.isOpened) {
			setTimeout(() => folderNameElement.querySelector('input').focus(), 200)
		}
	})

	/**
	 *
	 * @param {SubmitEvent} event
	 */
	const validateFolderName = (event) => {
		event.preventDefault()

		/**
		 * @type {string}
		 */
		const value = event.currentTarget.value

		setErrFolderName(
			value.includes('/') ? 'Folder name cannot have a "/" symbol' : null
		)

		setFolderName(value)
	}

	const onClose = () => {
		setErrFolderName(null)
		setFolderName('')
		props.onClose()
	}

	const onCreate = async () => {
		const foldeName = folderName()
		onClose()
		await props.onCreate(foldeName)
	}

	return (
		<>
			<Dialog open={props.isOpened} onClose={onClose}>
				<form onSubmit={onCreate}>
					<DialogTitle>Create folder</DialogTitle>
					<DialogContent>
						<TextField
							ref={folderNameElement}
							value={folderName()}
							required
							margin="dense"
							id="folder-name"
							label="New folder name"
							onChange={validateFolderName}
							helperText={errFolderName}
							error={errFolderName() !== null}
							fullWidth
							variant="standard"
						/>
					</DialogContent>
					<DialogActions>
						<Button
							type="submit"
							color="success"
							disabled={!folderName().length || errFolderName()}
						>
							Create
						</Button>
						<Button onClick={onClose} color="error">
							Cancel
						</Button>
					</DialogActions>
				</form>
			</Dialog>
		</>
	)
}

export default CreateFolderDialog
