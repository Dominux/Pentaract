import Box from "@suid/material/Box";
import Drawer from "@suid/material/Drawer";
import AppBar from "@suid/material/AppBar";
import Toolbar from "@suid/material/Toolbar";
import List from "@suid/material/List";
import CssBaseline from "@suid/material/CssBaseline";
import Typography from "@suid/material/Typography";
import Divider from "@suid/material/Divider";
import IconButton from "@suid/material/IconButton";
import ChevronLeftIcon from "@suid/icons-material/ChevronLeft";
import ChevronRightIcon from "@suid/icons-material/ChevronRight";
import ListItem from "@suid/material/ListItem";
import ListItemButton from "@suid/material/ListItemButton";
import InboxIcon from "@suid/icons-material/MoveToInbox";
import { children, createSignal } from "solid-js";
import { A, useNavigate } from "@solidjs/router";
import createLocalStore from "../../libs";
import Logout from "@suid/icons-material/Logout";
import NavBarSideBarItem from "./NavBarSideBarItem";

const initOpen = window.innerWidth > 840;

/**
 * @typedef {Object} NavBarProps
 * @property {import("solid-js").JSXElement[]} children
 */

/**
 *
 * @param {NavBarProps} props
 */
const NavBar = (props) => {
  const [open, setOpen] = createSignal(initOpen);
  const [_store, setStore] = createLocalStore();
  const navigate = useNavigate();
  const c = children(() => props.children);

  const logout = (_) => {
    setStore("access_token");
    setStore("redirect", "/");

    navigate("/login");
  };

  const toggleDrawerOpen = () => {
    setOpen((open) => !open);
  };

  return (
    <Box sx={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      <CssBaseline />

      <AppBar position="static" sx={{ width: "100vw" }}>
        <Toolbar sx={{ justifyContent: "space-between" }}>
          <Typography variant="h4" noWrap component="div">
            <A href="/">Pentaract</A>
          </Typography>

          <IconButton onClick={logout}>
            <Logout />
          </IconButton>
        </Toolbar>
      </AppBar>

      <Box sx={{ display: "flex", height: "100%" }}>
        <Drawer
          variant="permanent"
          open
          classes={{
            paper: open()
              ? "drawer-paper drawer-paper-opened"
              : "drawer-paper drawer-paper-closed",
          }}
        >
          <List>
            <ListItem disablePadding sx={{ display: "block" }}>
              <ListItemButton
                sx={{
                  justifyContent: open() ? "end" : "center",
                  py: 0.5,
                  px: 1,
                }}
                onClick={toggleDrawerOpen}
              >
                <IconButton>
                  {open() ? <ChevronLeftIcon /> : <ChevronRightIcon />}
                </IconButton>
              </ListItemButton>
            </ListItem>
          </List>
          <Divider />
          <List>
            <NavBarSideBarItem text="Storages" link="/storages" isFull={open()}>
              <InboxIcon />
            </NavBarSideBarItem>
            <NavBarSideBarItem
              text="Storage workers"
              link="/storages_workers"
              isFull={open()}
            >
              <InboxIcon />
            </NavBarSideBarItem>
          </List>
        </Drawer>

        <Box component="main" sx={{ p: 4 }}>
          {c()}
        </Box>
      </Box>
    </Box>
  );
};

export default NavBar;
