import { useBeforeLeave, useParams } from "@solidjs/router";
import { Show, createSignal, mapArray, onMount } from "solid-js";
import List from "@suid/material/List";
import MenuItem from "@suid/material/MenuItem";
import ListItemIcon from "@suid/material/ListItemIcon";
import ListItemText from "@suid/material/ListItemText";
import UploadFileIcon from "@suid/icons-material/UploadFile";
import UploadFolderIcon from "@suid/icons-material/DriveFolderUpload";
import Grid from "@suid/material/Grid";
import Stack from "@suid/material/Stack";
import Typography from "@suid/material/Typography";
import { Divider } from "@suid/material";

import API from "../../api";
import FSListItem from "../../components/FSListItem";
import Menu from "../../components/Menu";
import CreateFolderDialog from "../../components/CreateFolderDialog";

const Files = () => {
  /**
   * @type {[import("solid-js").Accessor<import("../../api").FSElement[]>, any]}
   */
  const [fsLayer, setFsLayer] = createSignal([]);
  const [isCreateFolderDialogOpen, setIsCreateFolderDialogOpen] =
    createSignal(false);
  const params = useParams();

  let uploadFileInputElement;

  const fetchFSLayer = async (path = params.path) => {
    const fsLayerRes = await API.files.getFSLayer(params.id, path);

    if (path.length) {
      const parentPath = path.split("/").slice(0, -1).join("/");
      const backToParent = { is_file: false, name: "..", path: parentPath };

      fsLayerRes.splice(0, 0, backToParent);
    }

    setFsLayer(fsLayerRes);
  };

  onMount(fetchFSLayer);

  useBeforeLeave(async (e) => {
    const basePath = `/storages/${params.id}`;
    if (e.to.startsWith(basePath)) {
      let newPath = e.to.slice(basePath.length);

      if (newPath.startsWith("/")) {
        newPath = newPath.slice(1);
      }

      console.log(newPath);
      await fetchFSLayer(newPath);
    }
  });

  const openCreateFolderDialog = () => {
    setIsCreateFolderDialogOpen(true);
  };
  const closeCreateFolderDialog = () => {
    setIsCreateFolderDialogOpen(false);
  };

  /**
   *
   * @param {string} folderName
   */
  const createFolder = async (folderName) => {
    const basePath = params.path.endsWith("/")
      ? params.path.slice(0, -1)
      : params.path;

    await API.files.createFolder(params.id, basePath, folderName);
    await fetchFSLayer();
  };

  const uploadFileClickHandler = () => {
    uploadFileInputElement.click();
  };

  /**
   *
   * @param {Event} event
   */
  const uploadFile = async (event) => {
    const file = event.target.files[0];
    if (file === undefined) {
      return;
    }

    await API.files.uploadFile(params.id, params.path, event.target.files[0]);
    await fetchFSLayer();
  };

  return (
    <>
      <Stack container>
        <Grid container sx={{ mb: 2 }}>
          <Grid item xs={6}>
            <Typography variant="h4">Files</Typography>
          </Grid>
          <Grid
            item
            xs={6}
            sx={{ display: "flex", justifyContent: "flex-end" }}
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
              <MenuItem>
                <ListItemIcon>
                  <UploadFileIcon />
                </ListItemIcon>
                <ListItemText>Upload file to</ListItemText>
              </MenuItem>
            </Menu>
          </Grid>
        </Grid>

        <Grid>
          <Show when={fsLayer().length} fallback={<>Not files yet</>}>
            <List sx={{ minWidth: 320, maxWidth: 540, mx: "auto" }}>
              <Divider />
              {mapArray(fsLayer, (fsElement) => (
                <>
                  <FSListItem fsElement={fsElement} storageId={params.id} />
                  <Divider />
                </>
              ))}
            </List>
          </Show>
        </Grid>
      </Stack>

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
    </>
  );
};

export default Files;
