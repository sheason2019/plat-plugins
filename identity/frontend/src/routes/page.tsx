import IdentityCard from "./components/identity-card";
import useDaemonContext from "./hooks/use-daemon-context";

export default function IndexPage() {
  const { data } = useDaemonContext();

  return (
    <div className="container mx-auto px-3">
      {JSON.stringify(data)}
      {/* <IdentityCard  /> */}
    </div>
  );
}
