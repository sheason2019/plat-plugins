import { createBrowserRouter, RouterProvider } from "react-router-dom";
import IndexPage from "./routes/page";
import { NextUIProvider } from "@nextui-org/react";
import EditPage from "./routes/edit/page";

const router = createBrowserRouter([
  {
    path: "/",
    element: <IndexPage />,
  },
  {
    path: "/edit",
    element: <EditPage />,
  },
]);

function App() {
  return (
    <NextUIProvider>
      <RouterProvider router={router} />
    </NextUIProvider>
  );
}

export default App;
