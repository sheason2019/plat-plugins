import { Identity } from "../../typings/core";

interface Props {
  identity: Identity;
}

export default function IdentityCard({ identity }: Props) {
  return (
    <div>
      <p>Avatar: {identity.avatar_url}</p>
      <p>Name: {identity.username}</p>
      <p>publicKey: {identity.public_key}</p>
      <p>updatedAt: {identity.updated_at}</p>
    </div>
  );
}
