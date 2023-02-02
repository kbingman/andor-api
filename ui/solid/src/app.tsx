import { lazy } from "solid-js";
import type { Component } from "solid-js";
import { useRoutes } from "@solidjs/router";

const routes = [
  {
    path: "/",
    component: lazy(() => import("./pages/index")),
  },
  // {
  //   path: "/episodes",
  //   component: lazy(() => import("./pages/episodes")),
  // },
];

const App: Component = () => {
  const Routes = useRoutes(routes);

  return (
    <div class="">
      <header class="p-2 bg-zinc-900 text-gray-50 shadow">
        <h1 class="text-lg font-bold">Scarif</h1>
      </header>
      <Routes />
    </div>
  );
};

export default App;
