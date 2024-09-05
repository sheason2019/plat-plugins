import { Avatar, Button, Input } from "@nextui-org/react";
import Header from "../components/header";
import useDaemonContext from "../hooks/use-daemon-context";
import useIdentity from "../hooks/use-identity";
import { useEffect, useState } from "react";
import { Identity } from "../../typings/core";
import axios from "axios";
import useSWRMutation from "swr/mutation";
import { toast } from "sonner";
import { useNavigate } from "react-router-dom";

export default function EditPage() {
  const navigate = useNavigate();
  const { data: daemonContext } = useDaemonContext();
  const { data: identity, mutate } = useIdentity(
    daemonContext.daemon.public_key,
    {
      suspense: true,
    }
  );

  const [value, setValue] = useState<Identity>();
  useEffect(() => {
    setValue(identity);
  }, [identity]);

  const { trigger } = useSWRMutation(
    "identity/put",
    async () =>
      axios.put("/api/identity/" + identity?.public_key, JSON.stringify(value)),
    {
      onSuccess() {
        toast("修改用户信息成功");
        mutate();
        navigate("/");
      },
    }
  );

  return (
    <>
      <Header title="编辑用户信息" />
      <div className="container max-w-xs mx-auto px-3 mt-3">
        <div className="flex flex-col items-center gap-2">
          <Avatar className="h-20 w-20" />
          <Input
            label="用户名"
            value={value?.username}
            onChange={(e) =>
              value && setValue({ ...value, username: e.target.value })
            }
          />
        </div>
        <Button
          className="w-full mt-4"
          color="primary"
          onClick={() => trigger()}
        >
          保存
        </Button>
      </div>
    </>
  );
}
