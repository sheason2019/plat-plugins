import { Spinner } from "@nextui-org/react";
import { PropsWithChildren } from "react";
import useInitial from "./use-initial";

export default function InitialProvider({ children }: PropsWithChildren) {
  const { isLoading } = useInitial();

  if (isLoading) {
    return (
      <div className="fixed inset-0 flex flex-col justify-center items-center">
        <Spinner label="正在初始化身份模块" labelColor="primary" />
      </div>
    );
  }

  return <>{children}</>;
}
