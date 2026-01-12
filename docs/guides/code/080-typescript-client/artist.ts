import type { ListResponse, SingleResponse } from "@decodelabs/underlay";

export interface ArtistDto {
  artistId: string;
  email: string;
  displayName: string;
  createdAt: string;
}

export type ArtistListResponse = ListResponse<ArtistDto>;
export type ArtistSingleResponse = SingleResponse<ArtistDto>;

export interface CreateArtistRequest {
  email: string;
  displayName: string;
}

export interface PaginationParams {
  limit?: number;
  offset?: number;
}
