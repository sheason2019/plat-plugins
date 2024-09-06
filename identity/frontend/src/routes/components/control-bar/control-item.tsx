import { Link } from "@nextui-org/react";
import { ReactNode } from "react";

interface Props {
  icon: ReactNode;
  label: ReactNode;
  href?: string;
}

export default function ControlItem({ icon, label, href }: Props) {
  return (
    <Link
      isBlock
      color="foreground"
      className="h-14 flex flex-col aspect-square text-gray-500"
      href={href}
    >
      <div className="aspect-square flex-1 flex justify-center items-center">
        {icon}
      </div>
      <div className="text-xs">{label}</div>
    </Link>
  );
}
