import ListItem from "@suid/material/ListItem";
import ListItemButton from "@suid/material/ListItemButton";
import ListItemIcon from "@suid/material/ListItemIcon";
import ListItemText from "@suid/material/ListItemText";
import FileIcon from "@suid/icons-material/InsertDriveFileOutlined";
import FolderIcon from "@suid/icons-material/Folder";
import { Show } from "solid-js";
import { A } from "@solidjs/router";

/**
 * @typedef {Object} FSListItemProps
 * @property {import("../../api").FSElement} fsElement
 * @property {string} storageId
 */

/**
 *
 * @param {import("../../api").FSElement} props
 * @returns
 */
const fsListItem = (props) => {
  return (
    <ListItem disablePadding>
      <ListItemButton>
        <ListItemIcon>
          <Show when={props.is_file} fallback={<FolderIcon />}>
            <FileIcon />
          </Show>
        </ListItemIcon>
        <ListItemText primary={props.name} />
      </ListItemButton>
    </ListItem>
  );
};

/**
 *
 * @param {FSListItemProps} props
 * @returns
 */
const FSListItem = (props) => {
  return (
    <Show
      when={!props.fsElement.is_file}
      fallback={fsListItem(props.fsElement)}
    >
      <A href={`/storages/${props.storageId}/${props.fsElement.path}`}>
        {fsListItem(props.fsElement)}
      </A>
    </Show>
  );
};

export default FSListItem;
