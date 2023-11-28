import { A } from "@solidjs/router";
import ListItem from "@suid/material/ListItem";
import ListItemButton from "@suid/material/ListItemButton";
import ListItemIcon from "@suid/material/ListItemIcon";
import ListItemText from "@suid/material/ListItemText";
import { children } from "solid-js";

/**
 * @typedef {Object} SideBarItemProps
 * @property {string} text
 * @property {boolean} isFull
 * @property {string} link
 * @property {import("solid-js").JSXElement[]} children
 */

/**
 *
 * @param {SideBarItemProps} props
 */
const SideBarItem = (props) => {
  const c = children(() => props.children);

  return (
    <ListItem key={props.text} disablePadding sx={{ display: "block" }}>
      <A href={props.link}>
        <ListItemButton
          sx={{
            minHeight: 48,
            justifyContent: props.isFull ? "initial" : "center",
            px: 2.5,
          }}
        >
          <ListItemIcon
            sx={{
              minWidth: 0,
              mr: props.isFull ? 3 : "auto",
              justifyContent: "center",
            }}
          >
            {c()}
          </ListItemIcon>
          <ListItemText
            primary={props.text}
            sx={{ display: props.isFull ? "border-box" : "none" }}
          />
        </ListItemButton>
      </A>
    </ListItem>
  );
};

export default SideBarItem;
