import { useCallback, useState } from "react";
import WelcomeSection from "./sections/welcome";
import { InitialSections } from "./sections/typings";
import FormSection from "./sections/form";
import CompleteSection from "./sections/complete";

export default function InitialPage() {
  const [section, setSection] = useState(InitialSections.Welcome);

  const renderSection = useCallback(() => {
    switch (section) {
      case InitialSections.Welcome:
        return <WelcomeSection setSection={setSection} />;
      case InitialSections.Form:
        return <FormSection setSection={setSection} />;
      case InitialSections.Complete:
        return <CompleteSection />;
    }
  }, [section]);

  return (
    <div className="absolute inset-0 flex flex-col items-center justify-center">
      {renderSection()}
    </div>
  );
}
