import ControlBar from "./components/control-bar";
import IdentityHero from "./components/identity-hero";

export default function IndexPage() {
  return (
    <div className="container max-w-xs mx-auto px-3 mt-3">
      <IdentityHero />
      <div className="mt-3">
        <ControlBar />
      </div>
    </div>
  );
}
