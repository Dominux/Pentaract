import { Routes, Route } from "@solidjs/router";

import Login from "./pages/Login";
import Home from "./pages/Home";
import BasicLayout from "./layouts/Basic";
import Storages from "./pages/Storages";
import StorageCreateForm from "./pages/Storages/StorageCreateForm";
import AlertStack from "./components/AlertStack";

const App = () => {
  return (
    <>
      <Routes>
        <Route path="/login" component={Login} />

        <Route path="/" component={BasicLayout}>
          <Route path="/" component={Home} />
          <Route path="/storages" component={Storages} />
          <Route path="/storages/register" component={StorageCreateForm} />
        </Route>
      </Routes>

      <AlertStack />
    </>
  );
};

export default App;
