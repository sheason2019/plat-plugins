import { InitialSections, SectionProps } from "./typings";
import NextButton from "./components/next-button";

export default function WelcomeSection({ setSection }: SectionProps) {
  return (
    <div className="container max-w-xs w-full mx-auto flex flex-col items-center px-3">
      <h1 className="text-xl font-bold font-mono">Plat Identity Plugin</h1>
      <div className="mt-5 text-default-500 font-mono flex flex-col items-center gap-2">
        <p>Plat 身份插件尚未初始化</p>
        <p>请跟随用户指引</p>
        <p>初始化您的 Plat 身份信息</p>
      </div>
      <NextButton onClick={() => setSection(InitialSections.Form)} />
    </div>
  );
}
