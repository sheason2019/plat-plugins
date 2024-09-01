import { createBrowserRouter, RouterProvider } from "react-router-dom";
import IndexPage from "./routes/page";

const router = createBrowserRouter([
  {
    path: "/",
    element: <IndexPage />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
