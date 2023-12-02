import Typography from "@suid/material/Typography";
import Grid from "@suid/material/Grid";
import Stack from "@suid/material/Stack";
import Paper from "@suid/material/Paper";
import Table from "@suid/material/Table";
import TableBody from "@suid/material/TableBody";
import TableCell from "@suid/material/TableCell";
import TableContainer from "@suid/material/TableContainer";
import TableHead from "@suid/material/TableHead";
import TableRow from "@suid/material/TableRow";
import Button from "@suid/material/Button";
import { Show, createSignal, mapArray, onMount } from "solid-js";
import { useNavigate } from "@solidjs/router";

import API from "../../api";

const StorageWorkers = () => {
  /**
   * @type {[import("solid-js").Accessor<import("../../api").StorageWorker[]>, any]}
   */
  const [storageWorkers, setStorageWorkers] = createSignal([]);
  const navigate = useNavigate();

  onMount(async () => {
    const storageWorkers = await API.storageWorkers.listStorageWorkers();
    setStorageWorkers(storageWorkers);
  });

  return (
    <Stack container>
      <Grid container sx={{ mb: 2 }}>
        <Grid item xs={6}>
          <Typography variant="h4">Storage Workers</Typography>
        </Grid>
        <Grid item xs={6} sx={{ display: "flex", justifyContent: "flex-end" }}>
          <Button
            onClick={() => navigate("/storage_workers/register")}
            variant="contained"
            color="success"
          >
            Register new
          </Button>
        </Grid>
      </Grid>

      <Grid>
        <TableContainer component={Paper}>
          <Table sx={{ minWidth: 650 }}>
            <Show
              when={storageWorkers().length}
              fallback={<div>There's no storage workers yet</div>}
            >
              <TableHead>
                <TableRow>
                  <TableCell>Name</TableCell>
                  <TableCell>Storage</TableCell>
                  <TableCell>Token</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {mapArray(storageWorkers, (sw) => (
                  <TableRow
                    sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
                  >
                    <TableCell component="th" scope="row">
                      {sw.name}
                    </TableCell>
                    <TableCell>{sw.storage_id}</TableCell>
                    <TableCell>{sw.token}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Show>
          </Table>
        </TableContainer>
      </Grid>
    </Stack>
  );
};

export default StorageWorkers;
