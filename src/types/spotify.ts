export interface NowPlayingTrack {
  title: string
  artist: string
  album?: string
  albumImageUrl?: string
  isPlaying: boolean
  progressMs?: number
  durationMs?: number
}
