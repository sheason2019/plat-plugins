import { ReactNode } from "react";

interface Props {
  label: ReactNode;
  value: ReactNode;
}

export default function EditItem({ label, value }: Props) {
  return (
    <div className="flex justify-between items-center py-2 select-none">
      <div>{label}</div>
      <div className="text-gray-500">{value}</div>
    </div>
  );
}
