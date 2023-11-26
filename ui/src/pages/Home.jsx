import { onMount } from "solid-js";
import { checkAuth } from "../common/auth_guard";

const Home = () => {
  onMount(checkAuth);

  return <h1>Home</h1>;
};

export default Home;
