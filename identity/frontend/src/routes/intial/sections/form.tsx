import { Input } from "@nextui-org/react";
import { InitialSections, SectionProps } from "./typings";
import PrevButton from "./components/prev-button";
import NextButton from "./components/next-button";
import { Identity } from "../../../typings/core";
import axios from "axios";

export default function FormSection({ setSection }: SectionProps) {
  const handleSubmit = async (e: FormData) => {
    const identity: Identity = {
      public_key: "",
      username: e.get("username")?.toString() ?? "",
      avatar_url: "",
      x25519_public_keys: [],
      hosts: [],
    };
    await axios.put("/api/current", identity);
    setSection(InitialSections.Complete);
  };

  return (
    <form
      className="flex flex-col items-center max-w-xs w-full"
      onSubmit={(e) => {
        e.preventDefault();
        handleSubmit(new FormData(e.currentTarget));
      }}
    >
      <p className="font-mono">请定义您的用户名</p>
      <Input name="username" className="mt-3" />
      <div className="flex gap-2">
        <PrevButton onClick={() => setSection(InitialSections.Welcome)} />
        <NextButton type="submit" />
      </div>
    </form>
  );
}
