import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import { createSignal } from "solid-js";

const CreateFolderDialog = () => {
  const [open, setOpen] = createSignal(false);
  const [errFolderName, setErrFolderName] = createSignal(null);

  /**
   *
   * @param {SubmitEvent} event
   */
  const validateFolderName = (event) => {
    event.preventDefault();

    /**
     * @type {string}
     */
    const value = event.currentTarget.value;

    setErrFolderName(
      value.includes("/") ? 'Folder name cannot have a "/" symbol' : null
    );
  };

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  return (
    <>
      <Dialog open={open} onClose={handleClose}>
        <DialogTitle>Subscribe</DialogTitle>
        <DialogContent>
          <DialogContentText>
            To subscribe to this website, please enter your email address here.
            We will send updates occasionally.
          </DialogContentText>
          <TextField
            autoFocus
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
          <Button onClick={handleClose}>Create</Button>
        </DialogActions>
      </Dialog>
    </>
  );
};

export default CreateFolderDialog;
