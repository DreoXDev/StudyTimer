export interface Task {
  id: string
  title: string
  completed: boolean
  createdAt: string
  completedAt?: string
  sortOrder: number
}
