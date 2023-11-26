import { A, useNavigate } from "@solidjs/router";
import Logout from "@suid/icons-material/Logout";
import MenuIcon from "@suid/icons-material/Menu";
import AppBar from "@suid/material/AppBar";
import IconButton from "@suid/material/IconButton";
import Toolbar from "@suid/material/Toolbar";
import Typography from "@suid/material/Typography";
import createLocalStore from "../../libs";

const Header = () => {
  const [_store, setStore] = createLocalStore();
  const navigate = useNavigate();

  const logout = (_) => {
    setStore("access_token");
    setStore("redirect", "/");

    navigate("/login");
  };

  return (
    <AppBar position="fixed">
      <Toolbar>
        <IconButton size="large" edge="start" aria-label="menu" sx={{ mr: 2 }}>
          <MenuIcon />
        </IconButton>

        <Typography variant="h5" component="div" sx={{ flexGrow: 1 }}>
          <A class="headline" href="/">
            Pentaract
          </A>
        </Typography>

        <IconButton onClick={logout}>
          <Logout />
        </IconButton>
      </Toolbar>
    </AppBar>
  );
};

export default Header;
