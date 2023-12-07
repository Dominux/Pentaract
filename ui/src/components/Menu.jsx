import MenuMUI from "@suid/material/Menu";
import Button from "@suid/material/Button";
import { children, createSignal } from "solid-js";

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
  const c = children(() => props.children);
  const [anchorEl, setAnchorEl] = createSignal(null);
  const open = () => Boolean(anchorEl());
  const handleClose = () => {
    setAnchorEl(null);
  };

  return (
    <>
      <Button
        id="basic-button"
        aria-controls={open() ? "basic-menu" : undefined}
        aria-haspopup="true"
        aria-expanded={open() ? "true" : undefined}
        variant="outlined"
        color="success"
        onClick={(event) => {
          setAnchorEl(event.currentTarget);
        }}
      >
        {props.button_title}
      </Button>
      <MenuMUI
        id="basic-menu"
        anchorEl={anchorEl()}
        open={open()}
        onClose={handleClose}
        MenuListProps={{ "aria-labelledby": "basic-button" }}
      >
        {c()}
      </MenuMUI>
    </>
  );
};

export default Menu;
