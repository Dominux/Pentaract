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
import ListItemIcon from "@suid/material/ListItemIcon";
import ListItemText from "@suid/material/ListItemText";
import InboxIcon from "@suid/icons-material/MoveToInbox";
import MailIcon from "@suid/icons-material/Mail";
import { createSignal } from "solid-js";
import { A, useNavigate } from "@solidjs/router";
import createLocalStore from "../../libs";
import Logout from "@suid/icons-material/Logout";

const drawerWidth = 240;
const initOpen = window.innerWidth > 600 + drawerWidth;

const NavBar = () => {
  const [open, setOpen] = createSignal(initOpen);
  const [_store, setStore] = createLocalStore();
  const navigate = useNavigate();

  const logout = (_) => {
    setStore("access_token");
    setStore("redirect", "/");

    navigate("/login");
  };

  const toggleDrawerOpen = () => {
    setOpen((open) => !open);
  };

  return (
    <Box>
      <CssBaseline />

      <AppBar position="relative">
        <Toolbar sx={{ justifyContent: "space-between" }}>
          <Typography variant="h4" noWrap component="div">
            <A class="headline" href="/">
              Pentaract
            </A>
          </Typography>

          <IconButton onClick={logout}>
            <Logout />
          </IconButton>
        </Toolbar>
      </AppBar>

      <Box sx={{ display: "inline-flex" }}>
        <Drawer
          variant="permanent"
          open
          classes={{ paper: "drawer-paper" }}
          sx={{ width: open() ? drawerWidth : "initial" }}
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
            {["Inbox", "Starred", "Send email", "Drafts"].map((text, index) => (
              <ListItem key={text} disablePadding sx={{ display: "block" }}>
                <ListItemButton
                  sx={{
                    minHeight: 48,
                    justifyContent: open() ? "initial" : "center",
                    px: 2.5,
                  }}
                >
                  <ListItemIcon
                    sx={{
                      minWidth: 0,
                      mr: open() ? 3 : "auto",
                      justifyContent: "center",
                    }}
                  >
                    {index % 2 === 0 ? <InboxIcon /> : <MailIcon />}
                  </ListItemIcon>
                  <ListItemText
                    primary={text}
                    sx={{ display: open() ? "border-box" : "none" }}
                  />
                </ListItemButton>
              </ListItem>
            ))}
          </List>
          <Divider />
          <List>
            {["All mail", "Trash", "Spam"].map((text, index) => (
              <ListItem key={text} disablePadding sx={{ display: "block" }}>
                <ListItemButton
                  sx={{
                    minHeight: 48,
                    justifyContent: open() ? "initial" : "center",
                    px: 2.5,
                  }}
                >
                  <ListItemIcon
                    sx={{
                      minWidth: 0,
                      mr: open() ? 3 : "auto",
                      justifyContent: "center",
                    }}
                  >
                    {index % 2 === 0 ? <InboxIcon /> : <MailIcon />}
                  </ListItemIcon>
                  <ListItemText
                    primary={text}
                    sx={{ display: open() ? "border-box" : "none" }}
                  />
                </ListItemButton>
              </ListItem>
            ))}
          </List>
        </Drawer>

        <Box component="main" sx={{ p: 4 }}>
          <Typography paragraph>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua. Rhoncus
            dolor purus non enim praesent elementum facilisis leo vel. Risus at
            ultrices mi tempus imperdiet. Semper risus in hendrerit gravida
            rutrum quisque non tellus. Convallis convallis tellus id interdum
            velit laoreet id donec ultrices. Odio morbi quis commodo odio aenean
            sed adipiscing. Amet nisl suscipit adipiscing bibendum est ultricies
            integer quis. Cursus euismod quis viverra nibh cras. Metus vulputate
            eu scelerisque felis imperdiet proin fermentum leo. Mauris commodo
            quis imperdiet massa tincidunt. Cras tincidunt lobortis feugiat
            vivamus at augue. At augue eget arcu dictum varius duis at
            consectetur lorem. Velit sed ullamcorper morbi tincidunt. Lorem
            donec massa sapien faucibus et molestie ac.
          </Typography>
          <Typography paragraph>
            Consequat mauris nunc congue nisi vitae suscipit. Fringilla est
            ullamcorper eget nulla facilisi etiam dignissim diam. Pulvinar
            elementum integer enim neque volutpat ac tincidunt. Ornare
            suspendisse sed nisi lacus sed viverra tellus. Purus sit amet
            volutpat consequat mauris. Elementum eu facilisis sed odio morbi.
            Euismod lacinia at quis risus sed vulputate odio. Morbi tincidunt
            ornare massa eget egestas purus viverra accumsan in. In hendrerit
            gravida rutrum quisque non tellus orci ac. Pellentesque nec nam
            aliquam sem et tortor. Habitant morbi tristique senectus et.
            Adipiscing elit duis tristique sollicitudin nibh sit. Ornare aenean
            euismod elementum nisi quis eleifend. Commodo viverra maecenas
            accumsan lacus vel facilisis. Nulla posuere sollicitudin aliquam
            ultrices sagittis orci a.
          </Typography>
        </Box>
      </Box>
    </Box>
  );
};

export default NavBar;
