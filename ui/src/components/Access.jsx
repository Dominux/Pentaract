import Grid from '@suid/material/Grid'
import Paper from '@suid/material/Paper'
import Table from '@suid/material/Table'
import TableBody from '@suid/material/TableBody'
import TableCell from '@suid/material/TableCell'
import TableContainer from '@suid/material/TableContainer'
import TableHead from '@suid/material/TableHead'
import TableRow from '@suid/material/TableRow'
import IconButton from '@suid/material/IconButton'
import DeleteIcon from '@suid/icons-material/Delete'
import EditIcon from '@suid/icons-material/Edit'
import { Show, createSignal, mapArray, onMount } from 'solid-js'
import { useParams } from '@solidjs/router'

import createLocalStore from '../../libs'
import AccessTypeChip from './AccessTypeChip'
import API from '../api'
import ActionConfirmDialog from './ActionConfirmDialog'
import { alertStore } from './AlertStack'
import GrantAccess from './GrantAccess'

/**
 * @typedef {Object} AccessProps
 * @property {() => void} setIsGrantAccessVisible
 * @property {() => void} onMount
 * @property {import('../api').UserWithAccess[]} users
 * @property {() => Promise<void>} refetchUsers
 */

/**
 *
 * @param {AccessProps} props
 */
const Access = (props) => {
	const [selectedUserEmail, setSelectedUserEmail] = createSignal()
	const [isRestrictConfirmOpened, setIsRestrictConfirmOpened] =
		createSignal(false)
	const [isChangeAccessOpened, setIsChangeAccessOpened] = createSignal(false)
	const [store, _setStore] = createLocalStore()
	const { addAlert } = alertStore
	const params = useParams()

	onMount(props.onMount)

	const onEditButtonClicked = (email) => {
		setSelectedUserEmail(email)
		setIsChangeAccessOpened(true)
	}

	const onChangeAccess = async () => {
		setIsChangeAccessOpened(false)
		await props.refetchUsers()
	}

	const onDeleteButtonClicked = (email) => {
		setSelectedUserEmail(email)
		setIsRestrictConfirmOpened(true)
	}

	const onRestrict = async () => {
		const userID = props.users.find((u) => u.email === selectedUserEmail()).id

		await API.access.restrictAccess(params.id, userID)
		addAlert(
			`Restricted access for the user with email ${selectedUserEmail()}`,
			'success'
		)

		await props.refetchUsers()
	}

	return (
		<>
			<Grid>
				<TableContainer component={Paper}>
					<Table sx={{ minWidth: 650 }}>
						<Show
							when={props.users.length}
							fallback={<div>There's no users with access yet</div>}
						>
							<TableHead>
								<TableRow>
									<TableCell>Email</TableCell>
									<TableCell>Access Type</TableCell>
									<TableCell></TableCell>
								</TableRow>
							</TableHead>
							<TableBody>
								{mapArray(
									() => props.users,
									(user) => (
										<TableRow
											sx={{
												cursor: 'pointer',
												'&:last-child td, &:last-child th': { border: 0 },
											}}
										>
											<TableCell component="th" scope="row">
												{user.email}
											</TableCell>

											<TableCell>
												<AccessTypeChip at={user.access_type} />
											</TableCell>

											<TableCell>
												<IconButton
													disabled={store.user?.email === user.email}
													onClick={() => onEditButtonClicked(user.email)}
												>
													<EditIcon />
												</IconButton>

												<IconButton
													disabled={store.user?.email === user.email}
													onClick={() => onDeleteButtonClicked(user.email)}
												>
													<DeleteIcon />
												</IconButton>
											</TableCell>
										</TableRow>
									)
								)}
							</TableBody>
						</Show>
					</Table>
				</TableContainer>
			</Grid>

			<ActionConfirmDialog
				action="Restrict"
				actionDescription={`restrict access for the user with email "${selectedUserEmail()}"`}
				entity="access"
				isOpened={isRestrictConfirmOpened()}
				onCancel={() => setIsRestrictConfirmOpened(false)}
				onConfirm={onRestrict}
			/>

			<GrantAccess
				afterGrant={onChangeAccess}
				email={selectedUserEmail()}
				isVisible={isChangeAccessOpened()}
				onClose={() => setIsChangeAccessOpened(false)}
			/>
		</>
	)
}

export default Access
