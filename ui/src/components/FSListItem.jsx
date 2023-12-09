import ListItem from "@suid/material/ListItem";
import ListItemButton from "@suid/material/ListItemButton";
import ListItemIcon from "@suid/material/ListItemIcon";
import ListItemText from "@suid/material/ListItemText";
import MenuMUI from "@suid/material/Menu";
import MenuItem from "@suid/material/MenuItem";
import IconButton from "@suid/material/IconButton";
import FileIcon from "@suid/icons-material/InsertDriveFileOutlined";
import FolderIcon from "@suid/icons-material/Folder";
import MoreVertIcon from "@suid/icons-material/MoreVert";
import DownloadIcon from "@suid/icons-material/Download";
import { createSignal } from "solid-js";
import { useNavigate, useParams } from "@solidjs/router";
import API from "../api";

/**
 * @typedef {Object} FSListItemProps
 * @property {import("../api").FSElement} fsElement
 * @property {string} storageId
 */

/**
 *
 * @param {FSListItemProps} props
 * @returns
 */
const FSListItem = (props) => {
  const [moreAnchorEl, setMoreAnchorEl] = createSignal(null);
  const navigate = useNavigate();
  const params = useParams();

  const openMore = () => Boolean(moreAnchorEl());

  const handleCloseMore = () => {
    setMoreAnchorEl(null);
  };

  const handleNavigate = () => {
    if (!props.fsElement.is_file) {
      navigate(`/storages/${props.storageId}/files/${props.fsElement.path}`);
    }
  };

  const download = async () => {
    const blob = await API.files.download(params.id, props.fsElement.path);

    const href = URL.createObjectURL(blob);
    const a = Object.assign(document.createElement("a"), {
      href,
      style: "display: none",
      download: props.fsElement.name,
    });
    document.body.appendChild(a);

    a.click();
    URL.revokeObjectURL(href);
    a.remove();
  };

  return (
    <>
      <ListItem disablePadding>
        <ListItemButton onClick={handleNavigate}>
          <ListItemIcon>
            <Show when={props.fsElement.is_file} fallback={<FolderIcon />}>
              <FileIcon />
            </Show>
          </ListItemIcon>
          <ListItemText primary={props.fsElement.name} />
        </ListItemButton>
        <IconButton
          onClick={(event) => {
            setMoreAnchorEl(event.currentTarget);
          }}
        >
          <MoreVertIcon />
        </IconButton>
      </ListItem>
      <MenuMUI
        id="basic-menu"
        anchorEl={moreAnchorEl()}
        open={openMore()}
        onClose={handleCloseMore}
        MenuListProps={{ "aria-labelledby": "basic-button" }}
      >
        <MenuItem onClick={download} disabled={!props.fsElement.is_file}>
          <ListItemIcon>
            <DownloadIcon />
          </ListItemIcon>
          <ListItemText>Download</ListItemText>
        </MenuItem>
      </MenuMUI>
    </>
  );
};

export default FSListItem;
