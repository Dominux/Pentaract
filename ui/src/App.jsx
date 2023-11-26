import { Routes, Route } from "@solidjs/router";

import Login from "./pages/Login";
import Home from "./pages/Home";

const App = () => {
  return (
    <>
      <Routes>
        <Route path="/" component={Home}></Route>
        <Route path="/login" component={Login} />
      </Routes>
    </>
  );
};

export default App;
