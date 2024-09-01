import useSWR from "swr";

interface UseIdentityOpt {
  publicKey: string;
}

export default function useIdentity({ publicKey }: UseIdentityOpt) {
  return useSWR(["identity", publicKey], async () => {});
}
