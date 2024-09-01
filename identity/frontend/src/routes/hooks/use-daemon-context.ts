import axios from "axios";
import useSWR from "swr";

export default function useDaemonContext() {
  return useSWR(
    "daemon",
    async () => {
      const res = await axios.get("/extern/daemon/context");
      return res.data;
    },
    { suspense: true }
  );
}
