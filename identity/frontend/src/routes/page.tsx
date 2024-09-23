import { Card, CardBody } from "@nextui-org/react";

export default function IndexPage() {
  return (
    <div className="container max-w-lg mx-auto px-3 grid grid-cols-12 gap-2 pt-3">
      <Card className="col-span-8 row-span-3">
        <CardBody>
          <p>用户名</p>
          <p>公钥</p>
          <p>头像</p>
        </CardBody>
      </Card>
      <Card className="col-span-4 row-span-2">
        <CardBody>
          <p>Identity</p>
          <p>版本</p>
        </CardBody>
      </Card>
      <Card className="col-span-4">
        <CardBody>点击编辑个人信息</CardBody>
      </Card>
      <Card className="col-span-6 row-span-4">
        <CardBody>日志信息</CardBody>
      </Card>
      <Card className="col-span-6 row-span-2">
        <CardBody>联系人 - 数量</CardBody>
      </Card>
      <Card className="col-span-6">
        <CardBody>服务器 - 已连接 / 总数</CardBody>
      </Card>
      <Card className="col-span-4">
        <CardBody>设置</CardBody>
      </Card>
      <Card className="col-span-2">
        <CardBody>关于</CardBody>
      </Card>
    </div>
  );
}
