import { Routes, Route } from "@solidjs/router";

import Login from "./pages/Login";
import Home from "./pages/Home";
import BasicLayout from "./layouts/Basic";
import Storages from "./pages/Storages";
import StorageCreateForm from "./pages/Storages/StorageCreateForm";
import AlertStack from "./components/AlertStack";
import StorageWorkers from "./pages/StorageWorkers";
import StorageWorkerCreateForm from "./pages/StorageWorkers/StorageWorkerCreateForm";
import Files from "./pages/Files";
import UploadFileTo from "./pages/Files/UploadFileTo";

const App = () => {
  return (
    <>
      <Routes>
        <Route path="/login" component={Login} />

        <Route path="/" component={BasicLayout}>
          <Route path="/" component={Home} />
          <Route path="/storages" component={Storages} />
          <Route path="/storages/register" component={StorageCreateForm} />
          <Route path="/storages/:id/files/*path" component={Files} />
          <Route path="/storages/:id/upload_to" component={UploadFileTo} />
          <Route path="/storage_workers" component={StorageWorkers} />
          <Route
            path="/storage_workers/register"
            component={StorageWorkerCreateForm}
          />
        </Route>
      </Routes>

      <AlertStack />
    </>
  );
};

export default App;
