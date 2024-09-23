import { createBrowserRouter, RouterProvider } from "react-router-dom";
import IndexPage from "./routes/page";
import { NextUIProvider } from "@nextui-org/react";
import { Toaster } from "sonner";
import InitialPage from "./routes/intial/page";
import IndexLayout from "./routes/layout";

const router = createBrowserRouter([
  {
    path: "/",
    element: <IndexLayout />,
    children: [
      {
        path: "",
        element: <IndexPage />,
      },
    ],
  },
  {
    path: "/initial",
    element: <InitialPage />,
  },
]);

function App() {
  return (
    <NextUIProvider>
      <Toaster />
      <RouterProvider router={router} />
    </NextUIProvider>
  );
}

export default App;
