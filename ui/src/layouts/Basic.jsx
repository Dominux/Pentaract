import { onMount } from "solid-js";
import { Outlet } from "@solidjs/router";
import Box from "@suid/material/Box";

import { checkAuth } from "../common/auth_guard";
import Header from "../components/Header";
import SideBar from "../components/SideBar";

const BasicLayout = () => {
  onMount(checkAuth);

  return (
    <Box sx={{ display: "flex" }}>
      <Header />
      <SideBar open={true} />
      <Outlet />
    </Box>
  );
};

export default BasicLayout;
