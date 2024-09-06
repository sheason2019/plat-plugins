import { Button, Card, CardBody, Divider } from "@nextui-org/react";
import { ReactNode } from "react";

interface Props {
  title: ReactNode;
}

export default function Header({ title }: Props) {
  return (
    <div className="mt-2 mx-4">
      <Card as="header" className="h-12 fixed">
        <CardBody className="p-1">
          <div className="flex items-center">
            <Button isIconOnly variant="light" onClick={() => history.back()}>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width={24}
                height={24}
                fill="currentColor"
                viewBox="0 0 16 16"
              >
                <path
                  fillRule="evenodd"
                  d="M12 8a.5.5 0 0 1-.5.5H5.707l2.147 2.146a.5.5 0 0 1-.708.708l-3-3a.5.5 0 0 1 0-.708l3-3a.5.5 0 1 1 .708.708L5.707 7.5H11.5a.5.5 0 0 1 .5.5z"
                />
              </svg>
            </Button>
            <Divider orientation="vertical" className="h-6 mr-2 ml-1" />
            <h1 className="mr-3 select-none">{title}</h1>
          </div>
        </CardBody>
      </Card>
      <div className="h-12" />
    </div>
  );
}
