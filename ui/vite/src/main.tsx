import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import App from "./app";
import "./index.css";

export async function loader() {
  const [people, episodes] = await Promise.all([
    (await fetch("http://localhost:3000/api/people")).json(),
    (await fetch("http://localhost:3000/api/episodes")).json(),
  ]);

  return { people, episodes };
}

const base = "";

const router = createBrowserRouter([
  {
    path: `/${base}`,
    element: <App />,
    loader: loader,
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
