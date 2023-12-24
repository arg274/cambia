import type { IArtistCredit } from "musicbrainz-api";

const baseUrl = "https://musicbrainz.org";
const caaBaseUrl = "https://coverartarchive.org";

export function getReleasesFromDiscId(discId: string): Promise<Response> {
    return fetch(`${baseUrl}/ws/2/discid/${discId}?fmt=json&inc=artists+labels`);
}

export function getCaaCovers(releaseId: string): Promise<Response> {
    return fetch(`${caaBaseUrl}/release/${releaseId}`);
}

export function getJoinedArtists(artists: IArtistCredit[] | undefined): string {
    if (!artists) {
        return "";
    }
    return artists.reduce((acc, artist, i) => acc + artist.name + (i < artists.length - 1 ? artist.joinphrase : ''), '')
}