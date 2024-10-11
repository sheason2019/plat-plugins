import axios from "axios";
import useSWR from "swr";
import { Identity } from "../typings/core";

export interface CurrentIdentity {
  identity: Identity;
}

export default function useCurrentIdentity() {
  return useSWR("identity", async () => {
    const resp = await axios.get<CurrentIdentity>("/api/current");
    return resp.data;
  });
}
