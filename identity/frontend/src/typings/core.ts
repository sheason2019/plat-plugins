export interface Identity {
  public_key: string;
  username: string;
  avatar_url: string;
  hosts: IdentityHost[];
}

export interface IdentityHost {
  identity_data_address: string;
  identity_page_address: string;
}
