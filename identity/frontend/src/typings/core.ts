export interface Identity {
  public_key: string;
  username: string;
  avatar_url: string;
  hosts: IdentityHost[];
  updated_at: number;
}

export interface IdentityHost {
  identity_data_address: string;
  identity_page_address: string;
}

export interface DaemonContext {
  public_key: string;
}
