import { useBeforeLeave, useNavigate, useParams } from '@solidjs/router'
import { Show, createSignal, mapArray, onCleanup, onMount } from 'solid-js'
import List from '@suid/material/List'
import MenuItem from '@suid/material/MenuItem'
import ListItemIcon from '@suid/material/ListItemIcon'
import ListItemText from '@suid/material/ListItemText'
import UploadFileIcon from '@suid/icons-material/UploadFile'
import UploadFolderIcon from '@suid/icons-material/DriveFolderUpload'
import FolderOpenIcon from '@suid/icons-material/FolderOpen'
import LockIcon from '@suid/icons-material/Lock'
import Grid from '@suid/material/Grid'
import Stack from '@suid/material/Stack'
import Typography from '@suid/material/Typography'
import Divider from '@suid/material/Divider'
import Fab from '@suid/material/Fab'
import ToggleButton from '@suid/material/ToggleButton'
import ToggleButtonGroup from '@suid/material/ToggleButtonGroup'
import AddIcon from '@suid/icons-material/Add'

import API from '../../api'
import FSListItem from '../../components/FSListItem'
import Menu from '../../components/Menu'
import CreateFolderDialog from '../../components/CreateFolderDialog'
import { alertStore } from '../../components/AlertStack'
import Access from '../../components/Access'
import GrantAccess from '../../components/GrantAccess'

const Files = () => {
	const { addAlert } = alertStore
	/**
	 * @type {[import("solid-js").Accessor<import("../../api").FSElement[]>, any]}
	 */
	const [fsLayer, setFsLayer] = createSignal([])
	/**
	 * @type {[import("solid-js").Accessor<import("../../api").Storage>, any]}
	 */
	const [storage, setStorage] = createSignal()
	const [isAccessPage, setIsAccessPage] = createSignal(false)
	const [isCreateFolderDialogOpen, setIsCreateFolderDialogOpen] =
		createSignal(false)
	const [isGrantAccessButtonVisible, setIsGrantButtonAccessVisible] =
		createSignal(false)
	const [isGrantAccessVisible, setIsGrantAccessVisible] = createSignal(false)
	/**
	 * @type {[import("solid-js").Accessor<import("../api").UserWithAccess[]>, any]}
	 */
	const [users, setUsers] = createSignal([])
	const navigate = useNavigate()
	const params = useParams()
	const basePath = `/storages/${params.id}/files`

	let uploadFileInputElement

	const fetchUsersWithAccess = async () => {
		try {
			const users = await API.access.listUsersWithAccess(params.id)
			setUsers(users)
			setIsGrantButtonAccessVisible(true)
		} catch (err) {
			addAlert('You do not have permissions to manage access', 'error')
			console.error(err)
			setIsGrantButtonAccessVisible(false)
		}
	}

	const fetchStorage = async () => {
		const storage = await API.storages.getStorage(params.id)
		setStorage(storage)
	}

	const fetchFSLayer = async (path = params.path) => {
		const fsLayerRes = await API.files.getFSLayer(params.id, path)

		if (path.length) {
			const parentPath = path.split('/').slice(0, -1).join('/')
			const backToParent = { is_file: false, name: '..', path: parentPath }

			fsLayerRes.splice(0, 0, backToParent)
		}

		setFsLayer(fsLayerRes)
	}

	const reload = async () => {
		if (window.location.pathname.startsWith(basePath)) {
			console.log(window.location.pathname)
			await fetchFSLayer()
		}
	}

	onMount(() => {
		Promise.all([fetchStorage(), fetchFSLayer()]).then()

		// Either me or the solidjs-router creator is dumb af so I have to use this sht
		window.addEventListener('popstate', reload, false)
	})

	onCleanup(() => window.removeEventListener('popstate', reload, false))

	useBeforeLeave(async (e) => {
		if (e.to.startsWith(basePath)) {
			let newPath = e.to.slice(basePath.length)

			if (newPath.startsWith('/')) {
				newPath = newPath.slice(1)
			}

			await fetchFSLayer(newPath)
		}
	})

	const openCreateFolderDialog = () => {
		setIsCreateFolderDialogOpen(true)
	}
	const closeCreateFolderDialog = () => {
		setIsCreateFolderDialogOpen(false)
	}

	/**
	 *
	 * @param {string} folderName
	 */
	const createFolder = async (folderName) => {
		const basePath = params.path.endsWith('/')
			? params.path.slice(0, -1)
			: params.path

		await API.files.createFolder(params.id, basePath, folderName)
		addAlert(`Created folder "${folderName}"`, 'success')
		await fetchFSLayer()
	}

	const uploadFileClickHandler = () => {
		uploadFileInputElement.click()
	}

	/**
	 *
	 * @param {Event} event
	 */
	const uploadFile = async (event) => {
		const file = event.target.files[0]
		if (file === undefined) {
			return
		}

		event.target.value = null

		await API.files.uploadFile(params.id, params.path, file)
		addAlert(`Uploaded file "${file.name}"`, 'success')
		await fetchFSLayer()
	}

	return (
		<>
			<Stack container>
				<Grid container sx={{ mb: 2 }}>
					<Grid item xs={4}>
						<Typography variant="h4">{storage()?.name}</Typography>
					</Grid>

					<Grid item xs={4}>
						<ToggleButtonGroup
							exclusive
							value={isAccessPage()}
							color="primary"
							onChange={(_, val) => setIsAccessPage(val)}
							sx={{ display: 'flex', justifyContent: 'center' }}
						>
							<ToggleButton value={false}>
								<FolderOpenIcon fontSize="small" />
								&nbsp; Files
							</ToggleButton>
							<ToggleButton value={true}>
								<LockIcon fontSize="small" />
								&nbsp; Access
							</ToggleButton>
						</ToggleButtonGroup>
					</Grid>

					<Grid
						item
						xs={4}
						sx={{ display: 'flex', justifyContent: 'flex-end' }}
					>
						<Show
							when={!isAccessPage()}
							fallback={
								<Show when={isGrantAccessButtonVisible()}>
									<Fab
										variant="extended"
										color="secondary"
										onClick={() => setIsGrantAccessVisible(true)}
									>
										<AddIcon sx={{ mr: 1 }} />
										Grant access
									</Fab>
									<GrantAccess
										isVisible={isGrantAccessVisible()}
										afterGrant={fetchUsersWithAccess}
										onClose={() => setIsGrantAccessVisible(false)}
									/>
								</Show>
							}
						>
							<Menu button_title="Create">
								<MenuItem onClick={openCreateFolderDialog}>
									<ListItemIcon>
										<UploadFolderIcon />
									</ListItemIcon>
									<ListItemText>Create folder</ListItemText>
								</MenuItem>
								<MenuItem onClick={uploadFileClickHandler}>
									<ListItemIcon>
										<UploadFileIcon />
									</ListItemIcon>
									<ListItemText>Upload file</ListItemText>
								</MenuItem>
								<MenuItem
									onClick={() => navigate(`/storages/${params.id}/upload_to`)}
								>
									<ListItemIcon>
										<UploadFileIcon />
									</ListItemIcon>
									<ListItemText>Upload file to</ListItemText>
								</MenuItem>
							</Menu>
						</Show>
					</Grid>
				</Grid>

				<Show
					when={!isAccessPage()}
					fallback={
						<Access
							setIsGrantAccessVisible={setIsGrantAccessVisible}
							users={users()}
							onMount={fetchUsersWithAccess}
							refetchUsers={fetchUsersWithAccess}
						/>
					}
				>
					<Grid>
						<Show when={fsLayer().length} fallback={<>Not files yet</>}>
							<List sx={{ minWidth: 320, maxWidth: 540, mx: 'auto' }}>
								<Divider />
								{mapArray(fsLayer, (fsElement) => (
									<>
										<FSListItem
											fsElement={fsElement}
											storageId={params.id}
											onDelete={fetchFSLayer}
										/>
										<Divider />
									</>
								))}
							</List>
						</Show>
					</Grid>

					<CreateFolderDialog
						isOpened={isCreateFolderDialogOpen()}
						onCreate={createFolder}
						onClose={closeCreateFolderDialog}
					/>
					<input
						ref={uploadFileInputElement}
						type="file"
						style="display: none"
						onChange={uploadFile}
					/>
				</Show>
			</Stack>
		</>
	)
}

export default Files
