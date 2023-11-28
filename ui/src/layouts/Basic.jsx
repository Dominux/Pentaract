import { onMount } from "solid-js";
import { Outlet } from "@solidjs/router";

import { checkAuth } from "../common/auth_guard";
import Header from "../components/Header";
import SideBar from "../components/SideBar";

const BasicLayout = () => {
  onMount(checkAuth);

  return (
    <>
      <Header />
      <SideBar>
        <Outlet />
      </SideBar>
    </>
  );
};

export default BasicLayout;
