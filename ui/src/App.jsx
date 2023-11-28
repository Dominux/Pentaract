import { Routes, Route } from "@solidjs/router";

import Login from "./pages/Login";
import Home from "./pages/Home";
import BasicLayout from "./layouts/Basic";
import Storages from "./pages/Storages";

const App = () => {
  return (
    <>
      <Routes>
        <Route path="/login" component={Login} />

        <Route path="/" component={BasicLayout}>
          <Route path="/" component={Home}></Route>
          <Route path="/storages" component={Storages}></Route>
        </Route>
      </Routes>
    </>
  );
};

export default App;
