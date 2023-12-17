import MenuMUI from '@suid/material/Menu'
import Fab from '@suid/material/Fab'
import AddIcon from '@suid/icons-material/Add'
import { children, createSignal } from 'solid-js'

/**
 * @typedef {Object} MenuProps
 * @property {string} button_title
 * @property {import("solid-js").JSXElement[]} children
 */

/**
 *
 * @param {MenuProps} props
 * @returns
 */
const Menu = (props) => {
	const c = children(() => props.children)
	const [anchorEl, setAnchorEl] = createSignal(null)
	const open = () => Boolean(anchorEl())
	const handleClose = () => {
		setAnchorEl(null)
	}

	return (
		<>
			<Fab
				id="basic-button"
				variant="extended"
				color="secondary"
				onClick={(event) => {
					setAnchorEl(event.currentTarget)
				}}
			>
				<AddIcon sx={{ mr: 1 }} />
				{props.button_title}
			</Fab>
			<MenuMUI
				id="basic-menu"
				anchorEl={anchorEl()}
				open={open()}
				onClose={handleClose}
				MenuListProps={{ 'aria-labelledby': 'basic-button' }}
			>
				{c()}
			</MenuMUI>
		</>
	)
}

export default Menu
