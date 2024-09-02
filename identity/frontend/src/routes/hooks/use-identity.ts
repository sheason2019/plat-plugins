import axios from "axios";
import useSWR, { SWRConfiguration } from "swr";
import { Identity } from "../../typings/core";

export default function useIdentity(
  publicKey: string,
  options?: SWRConfiguration
) {
  return useSWR(
    publicKey ? ["identity", publicKey] : null,
    async () => {
      const resp = await axios.get("/api/identity/" + publicKey);
      return resp.data as Identity;
    },
    options
  );
}
