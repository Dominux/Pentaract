import Drawer from "@suid/material/Drawer";
import List from "@suid/material/List";
import Divider from "@suid/material/Divider";
import IconButton from "@suid/material/IconButton";
import ChevronLeftIcon from "@suid/icons-material/ChevronLeft";
import ChevronRightIcon from "@suid/icons-material/ChevronRight";
import ListItem from "@suid/material/ListItem";
import ListItemButton from "@suid/material/ListItemButton";
import { createSignal } from "solid-js";
import StorageIcon from "@suid/icons-material/Storage";
import SmartToyIcon from "@suid/icons-material/SmartToyOutlined";

import SideBarItem from "./SideBarItem";

const initOpen = window.innerWidth > 840;

const SideBar = () => {
  const [open, setOpen] = createSignal(initOpen);

  const toggleDrawerOpen = () => {
    setOpen((open) => !open);
  };

  return (
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
        <SideBarItem text="Storages" link="/storages" isFull={open()}>
          <StorageIcon />
        </SideBarItem>
        <SideBarItem
          text="Storage workers"
          link="/storage_workers"
          isFull={open()}
        >
          <SmartToyIcon />
        </SideBarItem>
      </List>
    </Drawer>
  );
};

export default SideBar;
