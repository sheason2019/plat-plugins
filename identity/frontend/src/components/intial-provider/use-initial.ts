import axios from "axios";
import useSWR from "swr";
import { ICheckResponse } from "./typings";
import { useNavigate } from "react-router-dom";

export default function useInitial() {
  const navigate = useNavigate();

  return useSWR(
    "check",
    async () => {
      const res = await axios.get("/api/check");
      return res.data as ICheckResponse;
    },
    {
      onSuccess(data) {
        if (data.should_init) {
          navigate("/initial");
        }
      },
    }
  );
}
