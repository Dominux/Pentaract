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
import { Show, mapArray } from "solid-js";
import { useNavigate } from "@solidjs/router";

function createData(name, calories, fat, carbs, protein) {
  return { name, calories, fat, carbs, protein };
}

const rows = [
  createData("Frozen yoghurt", 159, 6.0, 24, 4.0),
  createData("Ice cream sandwich", 237, 9.0, 37, 4.3),
  createData("Eclair", 262, 16.0, 24, 6.0),
  createData("Cupcake", 305, 3.7, 67, 4.3),
  createData("Gingerbread", 356, 16.0, 49, 3.9),
];

const Storages = () => {
  const navigate = useNavigate();

  return (
    <Stack container>
      <Grid container sx={{ mb: 2 }}>
        <Grid item xs={6}>
          <Typography variant="h4">Storages</Typography>
        </Grid>
        <Grid item xs={6} sx={{ display: "flex", justifyContent: "flex-end" }}>
          <Button
            onClick={() => navigate("/storages/register")}
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
              when={rows.length}
              fallback={<div>There's no storages yet</div>}
            >
              <TableHead>
                <TableRow>
                  <TableCell>Name</TableCell>
                  <TableCell>Size</TableCell>
                  <TableCell>Files</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {mapArray(
                  () => rows,
                  (row) => (
                    <TableRow
                      sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
                    >
                      <TableCell component="th" scope="row">
                        {row.name}
                      </TableCell>
                      <TableCell>{row.calories}</TableCell>
                      <TableCell>{row.fat}</TableCell>
                    </TableRow>
                  )
                )}
              </TableBody>
            </Show>
          </Table>
        </TableContainer>
      </Grid>
    </Stack>
  );
};

export default Storages;
