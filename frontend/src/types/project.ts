export interface Project {
  id: string;
  name: string;
  client_label: string | null;
  status: string;
  created_at: string;
  updated_at: string;
  last_used_at: string | null;
  archived_at: string | null;
}
