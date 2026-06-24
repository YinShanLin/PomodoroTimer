import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export interface TimerStatus {
  state: 'Work' | 'ShortBreak' | 'LongBreak' | 'Idle'
  remaining_secs: number
  total_secs: number
  session: number
  sessions_until_long: number
  is_paused: boolean
  next_action: 'Work' | 'Break' | 'None'
}

export interface TimerSettings {
  work_duration: number
  short_break: number
  long_break: number
  sessions_before_long: number
  sound_enabled: boolean
}

export interface PomodoroRecord {
  start_time: string
  duration_secs: number
  state: string
  completed: boolean
}

export interface TodayStats {
  count: number
  total_minutes: number
}

const status = ref<TimerStatus>({
  state: 'Idle',
  remaining_secs: 25 * 60,
  total_secs: 25 * 60,
  session: 0,
  sessions_until_long: 4,
  is_paused: false,
  next_action: 'None',
})

const settings = ref<TimerSettings>({
  work_duration: 25 * 60,
  short_break: 5 * 60,
  long_break: 15 * 60,
  sessions_before_long: 4,
  sound_enabled: true,
})

const todayStats = ref<TodayStats>({ count: 0, total_minutes: 0 })
const showSettings = ref(false)
const currentView = ref<'timer' | 'stats'>('timer')

let unlistenStatus: (() => void) | null = null
let unlistenAlert: (() => void) | null = null

export function useTimer() {
  const formatTime = (secs: number): string => {
    const m = Math.floor(secs / 60)
    const s = secs % 60
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`
  }

  const stateLabel = (): string => {
    switch (status.value.state) {
      case 'Work': return '专注中'
      case 'ShortBreak': return '短休息'
      case 'LongBreak': return '长休息'
      case 'Idle':
        if (status.value.next_action === 'Break') return '准备休息'
        if (status.value.next_action === 'Work') return '准备专注'
        return '准备开始'
    }
  }

  const progress = (): number => {
    if (status.value.total_secs === 0) return 0
    return 1 - status.value.remaining_secs / status.value.total_secs
  }

  const start = async () => {
    await invoke('start_timer')
  }

  const startBreak = async (isLong: boolean) => {
    await invoke('start_break', { isLong })
  }

  const pause = async () => {
    await invoke('pause_timer')
  }

  const skip = async () => {
    await invoke('skip_timer')
  }

  const reset = async () => {
    await invoke('reset_timer')
  }

  const updateSettings = async (newSettings: TimerSettings) => {
    settings.value = newSettings
    await invoke('update_settings', { settings: newSettings })
  }

  const refreshStats = async () => {
    const stats = await invoke<TodayStats>('get_today_stats')
    todayStats.value = stats
  }

  const getRecords = async (): Promise<PomodoroRecord[]> => {
    return await invoke<PomodoroRecord[]>('get_records')
  }

  onMounted(async () => {
    const initialStatus = await invoke<TimerStatus>('get_status')
    status.value = initialStatus

    const initialSettings = await invoke<TimerSettings>('get_settings')
    settings.value = initialSettings

    await refreshStats()

    unlistenStatus = await listen<TimerStatus>('timer-status', (event) => {
      status.value = event.payload
    })

    unlistenAlert = await listen('timer-alert', () => {
      refreshStats()
    })
  })

  onUnmounted(() => {
    unlistenStatus?.()
    unlistenAlert?.()
  })

  return {
    status,
    settings,
    todayStats,
    showSettings,
    currentView,
    formatTime,
    stateLabel,
    progress,
    start,
    startBreak,
    pause,
    skip,
    reset,
    updateSettings,
    refreshStats,
    getRecords,
  }
}
