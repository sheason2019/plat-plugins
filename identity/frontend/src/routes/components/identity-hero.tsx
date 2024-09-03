import { Avatar, Card, CardBody } from "@nextui-org/react";
import useDaemonContext from "../hooks/use-daemon-context";
import useIdentity from "../hooks/use-identity";

export default function IdentityHero() {
  const { data: daemonContext } = useDaemonContext();
  const { data: identity } = useIdentity(daemonContext.public_key, {
    suspense: true,
  });

  return (
    <Card>
      <CardBody>
        <div className="flex items-center gap-3">
          <Avatar size="lg" />
          <div className="flex-1 flex flex-col overflow-hidden gap-1">
            <p className="font-bold">{identity?.username}</p>
            <p className="text-gray-500 text-xs whitespace-nowrap text-ellipsis overflow-hidden">
              ID: {identity?.public_key}
            </p>
          </div>
        </div>
      </CardBody>
    </Card>
  );
}
