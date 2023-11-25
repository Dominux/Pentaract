import type { Component } from "solid-js";
import { Routes, Route, Navigate } from "@solidjs/router";

import Login from "./pages/Login";

const App: Component = () => {
  return (
    <>
      <Routes>
        <Route path="/login" component={Login} />
        <Route path="/" element={<Navigate href={"/login"} />}></Route>
        <Route
          path="/about"
          element={<div>This site was made with Solid</div>}
        />
      </Routes>
    </>
  );
};

export default App;
