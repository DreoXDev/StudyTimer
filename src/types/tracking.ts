export interface TrackingEvent {
  id: string
  eventType: string
  startedAt: string
  endedAt?: string
  durationSeconds?: number
  value?: number
  unit?: string
  source: string
  note?: string
  metadataJson?: string
  createdAt: string
  updatedAt: string
  deletedAt?: string
  syncedAt?: string
  syncStatus: string
}

export interface CreateTrackingEventPayload {
  id: string
  eventType: string
  startedAt: string
  endedAt?: string
  durationSeconds?: number
  value?: number
  unit?: string
  source?: string
  note?: string
  metadataJson?: string
}

export interface BucketValue {
  bucket: string
  seconds: number
  count: number
}

export interface StudySummary {
  totalSeconds: number
  completedSessions: number
  interruptedSessions: number
  averageSessionSeconds: number
  byBucket: BucketValue[]
}

export interface SmokingSummary {
  totalCigarettes: number
  byBucket: BucketValue[]
}

export interface TrackingSummary {
  rangeStart: string
  rangeEnd: string
  study: StudySummary
  smoking: SmokingSummary
}
