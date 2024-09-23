import { Button, Link } from "@nextui-org/react";

export default function CompleteSection() {
  return (
    <div className="container max-w-xs w-full mx-auto flex flex-col items-center px-3">
      <h1 className="text-xl font-bold font-mono">初始化已完成</h1>
      <Button className="mt-4" as={Link} href="/" color="primary">
        回到首页
      </Button>
    </div>
  );
}
