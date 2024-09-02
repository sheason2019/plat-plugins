import { Avatar, Button } from "@nextui-org/react";
import EditItem from "./components/edit-item";

export default function EditPage() {
  return (
    <div className="container max-w-xs mx-auto px-3 mt-3">
      <EditItem label="用户头像" value={<Avatar />} />
      <EditItem label="用户名" value="123" />
      <Button className="w-full" color="primary">
        保存
      </Button>
    </div>
  );
}
