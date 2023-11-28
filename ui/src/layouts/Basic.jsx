import { onMount } from "solid-js";
import { Outlet } from "@solidjs/router";
import Box from "@suid/material/Box";

import { checkAuth } from "../common/auth_guard";
import NavBar from "../components/NavBar";

const BasicLayout = () => {
  onMount(checkAuth);

  return (
    <Box sx={{ display: "flex" }}>
      <NavBar>
        <Outlet />
      </NavBar>
    </Box>
  );
};

export default BasicLayout;
