import { Component } from "solid-js";
import Container from "@suid/material/Container";
import Box from "@suid/material/Box";
import TextField from "@suid/material/TextField";
import Button from "@suid/material/Button";
import Paper from "@suid/material/Paper";
import Typography from "@suid/material/Typography";
import Divider from "@suid/material/Divider";
import createLocalStore from "../../libs";
import API from "../api";

const Login: Component = () => {
  const [_store, setStore] = createLocalStore();

  const handleSubmit = async (event: SubmitEvent) => {
    event.preventDefault();
    const data = new FormData(event.currentTarget);

    const loginResponse = await API.auth.login(
      data.get("username"),
      data.get("password")
    );

    setStore("access_token", loginResponse.access_token);
  };

  return (
    <Container maxWidth="sm" sx={{ width: "fit-content" }}>
      <Paper sx={{ mt: "20vh" }} elevation={4}>
        <Box
          component="form"
          onSubmit={handleSubmit}
          sx={{
            px: 5,
            py: 2,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            "& > :not(style)": { my: 1.5 },
          }}
        >
          <Typography variant="h5">Pentaract Account</Typography>
          <Divider />
          <TextField
            name="username"
            label="Username"
            variant="standard"
            required
          />
          <TextField
            name="password"
            label="Password"
            variant="standard"
            type="password"
            required
          />
          <Divider />
          <Button type="submit" variant="contained">
            Login
          </Button>
        </Box>
      </Paper>
    </Container>
  );
};

export default Login;
