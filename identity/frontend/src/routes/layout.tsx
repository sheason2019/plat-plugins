import { Outlet } from "react-router-dom";
import InitialProvider from "../components/intial-provider";

export default function IndexLayout() {
  return (
    <InitialProvider>
      <Outlet />
    </InitialProvider>
  );
}
