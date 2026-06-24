<script setup lang="ts">
import { useTimer } from './composables/useTimer'
import TimerRing from './components/TimerRing.vue'
import ControlPanel from './components/ControlPanel.vue'
import SettingsPanel from './components/SettingsPanel.vue'
import StatsPanel from './components/StatsPanel.vue'

const {
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
} = useTimer()

const handleSave = async (newSettings: typeof settings.value) => {
  await updateSettings(newSettings)
  showSettings.value = false
}
</script>

<template>
  <div class="app">
    <header class="header">
      <div class="logo">
        <span class="logo-icon">🍅</span>
        <span class="logo-text">番茄钟</span>
      </div>
      <nav class="nav-tabs">
        <button
          class="nav-tab"
          :class="{ active: currentView === 'timer' }"
          @click="currentView = 'timer'"
        >
          计时
        </button>
        <button
          class="nav-tab"
          :class="{ active: currentView === 'stats' }"
          @click="currentView = 'stats'"
        >
          统计
        </button>
      </nav>
      <button class="settings-btn" @click="showSettings = true">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3" />
          <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" />
        </svg>
      </button>
    </header>

    <main class="main">
      <template v-if="currentView === 'timer'">
        <div class="timer-container">
          <div class="state-badge" :class="status.state.toLowerCase()">
            {{ stateLabel() }}
          </div>

          <TimerRing :progress="progress()" :state="status.state">
            <div class="time-display">{{ formatTime(status.remaining_secs) }}</div>
          </TimerRing>

          <div class="session-dots" v-if="status.session > 0">
            <div
              v-for="i in settings.sessions_before_long"
              :key="i"
              class="session-dot"
              :class="{ active: i <= status.session }"
            />
          </div>

          <ControlPanel
            :state="status.state"
            :is-paused="status.is_paused"
            :next-action="status.next_action"
            :session="status.session"
            :sessions-before-long="settings.sessions_before_long"
            @start="start"
            @start-break="startBreak"
            @pause="pause"
            @skip="skip"
            @reset="reset"
          />
        </div>

        <div class="today-stats" v-if="todayStats.count > 0">
          <div class="stat-item">
            <span class="stat-number">{{ todayStats.count }}</span>
            <span class="stat-label">个番茄</span>
          </div>
          <div class="stat-divider">·</div>
          <div class="stat-item">
            <span class="stat-number">{{ todayStats.total_minutes }}</span>
            <span class="stat-label">分钟</span>
          </div>
        </div>
      </template>

      <template v-else>
        <StatsPanel />
      </template>
    </main>

    <SettingsPanel
      v-if="showSettings"
      :settings="settings"
      @save="handleSave"
      @close="showSettings = false"
    />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #1E2A1F;
  color: #F8F4EC;
  overflow: hidden;
  user-select: none;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

#app {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

:root {
  --bg-primary: #1E2A1F;
  --bg-secondary: #2A3A2C;
  --bg-tertiary: #354A38;
  --surface: rgba(248, 244, 236, 0.05);
  --surface-hover: rgba(248, 244, 236, 0.08);
  --text-primary: #F8F4EC;
  --text-secondary: rgba(248, 244, 236, 0.7);
  --text-tertiary: rgba(248, 244, 236, 0.4);
  --accent-red: #E74C3C;
  --accent-red-dark: #C0392B;
  --accent-orange: #F39C12;
  --accent-green: #27AE60;
  --accent-green-dark: #1E8449;
  --border: rgba(248, 244, 236, 0.08);
  --shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  --radius-sm: 8px;
  --radius-md: 12px;
  --radius-lg: 16px;
  --radius-xl: 24px;
  --font-display: 'Playfair Display', Georgia, serif;
  --font-body: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  --font-mono: 'JetBrains Mono', 'SF Mono', monospace;
}
</style>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: linear-gradient(180deg, #1E2A1F 0%, #162017 100%);
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}

.logo {
  display: flex;
  align-items: center;
  gap: 6px;
}

.logo-icon {
  font-size: 18px;
}

.logo-text {
  font-family: var(--font-display);
  font-size: 16px;
  font-weight: 500;
  color: var(--text-primary);
}

.nav-tabs {
  display: flex;
  gap: 3px;
  background: var(--surface);
  padding: 3px;
  border-radius: var(--radius-md);
}

.nav-tab {
  background: none;
  border: none;
  padding: 6px 16px;
  font-family: var(--font-body);
  font-size: 12px;
  font-weight: 500;
  color: var(--text-tertiary);
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.nav-tab:hover {
  color: var(--text-secondary);
}

.nav-tab.active {
  color: var(--text-primary);
  background: var(--bg-tertiary);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.settings-btn {
  background: none;
  border: none;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 6px;
  border-radius: var(--radius-sm);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.settings-btn:hover {
  color: var(--text-primary);
  background: var(--surface);
}

.main {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 20px;
  padding: 20px 16px;
  overflow-y: auto;
  min-height: 0;
}

.timer-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  flex: 1;
  justify-content: center;
  min-height: 0;
  width: 100%;
  max-width: 400px;
}

.state-badge {
  font-family: var(--font-body);
  font-size: 12px;
  font-weight: 500;
  padding: 4px 14px;
  border-radius: var(--radius-xl);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  flex-shrink: 0;
}

.state-badge.work {
  background: rgba(231, 76, 60, 0.15);
  color: #E74C3C;
  border: 1px solid rgba(231, 76, 60, 0.2);
}

.state-badge.shortbreak,
.state-badge.longbreak {
  background: rgba(39, 174, 96, 0.15);
  color: #27AE60;
  border: 1px solid rgba(39, 174, 96, 0.2);
}

.state-badge.idle {
  background: var(--surface);
  color: var(--text-secondary);
  border: 1px solid var(--border);
}

.time-display {
  font-family: var(--font-mono);
  font-size: clamp(36px, 12vw, 56px);
  font-weight: 300;
  font-variant-numeric: tabular-nums;
  letter-spacing: 2px;
  color: var(--text-primary);
}

.session-dots {
  display: flex;
  gap: 6px;
  flex-shrink: 0;
}

.session-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--surface);
  border: 1px solid var(--border);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.session-dot.active {
  background: var(--accent-red);
  border-color: var(--accent-red);
  box-shadow: 0 0 10px rgba(231, 76, 60, 0.4);
}

.today-stats {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 20px;
  background: var(--surface);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border);
  flex-shrink: 0;
}

.stat-item {
  display: flex;
  align-items: baseline;
  gap: 3px;
}

.stat-number {
  font-family: var(--font-mono);
  font-size: 16px;
  font-weight: 500;
  color: var(--accent-orange);
}

.stat-label {
  font-size: 12px;
  color: var(--text-tertiary);
}

.stat-divider {
  color: var(--text-tertiary);
  font-size: 16px;
}

@media (max-height: 500px) {
  .header { padding: 8px 12px; }
  .main { gap: 12px; padding: 12px; }
  .timer-container { gap: 10px; }
  .state-badge { font-size: 11px; padding: 3px 10px; }
  .session-dots { gap: 4px; }
  .session-dot { width: 5px; height: 5px; }
  .today-stats { padding: 6px 14px; }
}

@media (max-width: 360px) {
  .logo-text { display: none; }
  .nav-tab { padding: 6px 12px; font-size: 11px; }
}
</style>
