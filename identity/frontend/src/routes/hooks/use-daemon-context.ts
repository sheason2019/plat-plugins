import axios from "axios";
import useSWR from "swr";
import { DaemonContext } from "../../typings/core";

export default function useDaemonContext() {
  return useSWR(
    "daemon",
    async () => {
      const res = await axios.get("/api/context");
      return res.data as DaemonContext;
    },
    { suspense: true }
  );
}
