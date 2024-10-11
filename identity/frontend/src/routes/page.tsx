import {
  Avatar,
  Card,
  CardBody,
  Divider,
  Listbox,
  ListboxItem,
} from "@nextui-org/react";
import useCurrentIdentity from "../hooks/use-current-identity";

export default function IndexPage() {
  const { data } = useCurrentIdentity();

  return (
    <div className="container max-w-lg mx-auto px-3 pt-10">
      <div className="grid grid-cols-12 gap-3">
        <Card className="col-span-12">
          <CardBody>
            <div className="flex items-center gap-2">
              <Avatar className="shrink-0" />
              <div className="flex-1 overflow-hidden">
                <p className="font-bold whitespace-nowrap text-ellipsis overflow-hidden">
                  {data?.identity.username}
                </p>
                <p className="text-sm text-default-500 whitespace-nowrap text-ellipsis overflow-hidden">
                  {data?.identity.public_key}
                </p>
              </div>
            </div>
          </CardBody>
        </Card>
        <Card className="col-span-6">
          <CardBody>
            <p>联系人</p>
            <p>0</p>
          </CardBody>
        </Card>
        <Card className="col-span-6">
          <CardBody>
            <p>服务器</p>
            <p>0/0</p>
          </CardBody>
        </Card>
      </div>
      <Divider className="my-5" />
      <Listbox>
        <ListboxItem key="log">日志</ListboxItem>
        <ListboxItem key="settings">设置</ListboxItem>
        <ListboxItem key="about">关于</ListboxItem>
      </Listbox>
    </div>
  );
}
