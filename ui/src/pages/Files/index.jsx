import { useBeforeLeave, useParams } from "@solidjs/router";
import { Show, createSignal, mapArray, onMount } from "solid-js";
import List from "@suid/material/List";
import Button from "@suid/material/Button";
import Grid from "@suid/material/Grid";
import Stack from "@suid/material/Stack";
import Typography from "@suid/material/Typography";
import { Divider } from "@suid/material";

import API from "../../api";
import FSListItem from "../../components/FSListItem";

const Files = () => {
  /**
   * @type {[import("solid-js").Accessor<import("../../api").FSElement[]>, any]}
   */
  const [fsLayer, setFsLayer] = createSignal([]);
  const params = useParams();

  const fetchFSLayer = async () => {
    console.log(params.path);

    const fsLayerRes = await API.files.getFSLayer(params.id, params.path);

    if (params.path.length) {
      const parentPath = params.path.split("/").slice(0, -1).join("/");
      const backToParent = { is_file: false, name: "..", path: parentPath };

      fsLayerRes.splice(0, 0, backToParent);
    }

    setFsLayer(fsLayerRes);
  };

  onMount(fetchFSLayer);

  useBeforeLeave((e) => {
    if (e.to.startsWith(`/storages/${params.id}`)) {
      e.retry(true);
    }
  });

  return (
    <Stack container>
      <Grid container sx={{ mb: 2 }}>
        <Grid item xs={6}>
          <Typography variant="h4">Files</Typography>
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
  );
};

export default Files;
