export interface Artist {
  id: string;
  email: string;
  displayName: string;
  createdAt: string;
}

export interface ArtistListResponse {
  data: Artist[];
  pagination?: {
    limit: number;
    offset: number;
    total: number;
  };
}

export interface CreateArtistRequest {
  email: string;
  displayName: string;
}

export interface PaginationParams {
  limit?: number;
  offset?: number;
}
