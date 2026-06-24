<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTimer } from '../composables/useTimer'
import type { PomodoroRecord } from '../composables/useTimer'

const { todayStats, getRecords, refreshStats } = useTimer()

const records = ref<PomodoroRecord[]>([])
const viewMode = ref<'today' | 'all'>('today')

const loadRecords = async () => {
  records.value = await getRecords()
}

const filteredRecords = () => {
  const completed = records.value.filter(r => r.completed && r.state === 'Work')
  if (viewMode.value === 'today') {
    const today = new Date().toISOString().split('T')[0]
    return completed.filter(r => r.start_time.startsWith(today))
  }
  return completed
}

const weeklyStats = () => {
  const completed = records.value.filter(r => r.completed && r.state === 'Work')
  const weekAgo = new Date()
  weekAgo.setDate(weekAgo.getDate() - 7)
  const weekRecords = completed.filter(r => new Date(r.start_time) >= weekAgo)

  const byDay: Record<string, number> = {}
  for (let i = 6; i >= 0; i--) {
    const d = new Date()
    d.setDate(d.getDate() - i)
    const key = d.toISOString().split('T')[0]
    byDay[key] = 0
  }

  weekRecords.forEach(r => {
    const day = r.start_time.split(' ')[0]
    if (day in byDay) {
      byDay[day]++
    }
  })

  return Object.entries(byDay).map(([date, count]) => ({
    date: date.slice(5),
    count,
  }))
}

const formatDuration = (secs: number) => {
  const hours = Math.floor(secs / 3600)
  const mins = Math.floor((secs % 3600) / 60)
  if (hours > 0) return `${hours}h ${mins}m`
  return `${mins}m`
}

const getMaxCount = () => {
  const stats = weeklyStats()
  return Math.max(...stats.map(s => s.count), 1)
}

onMounted(() => {
  loadRecords()
  refreshStats()
})
</script>

<template>
  <div class="stats-page">
    <div class="stats-header">
      <h2>统计概览</h2>
      <div class="view-toggle">
        <button
          class="toggle-btn"
          :class="{ active: viewMode === 'today' }"
          @click="viewMode = 'today'"
        >
          今日
        </button>
        <button
          class="toggle-btn"
          :class="{ active: viewMode === 'all' }"
          @click="viewMode = 'all'"
        >
          全部
        </button>
      </div>
    </div>

    <div class="stats-cards">
      <div class="stat-card">
        <div class="stat-icon">🍅</div>
        <div class="stat-value">{{ todayStats.count }}</div>
        <div class="stat-label">今日番茄</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon">⏱️</div>
        <div class="stat-value">{{ todayStats.total_minutes }}</div>
        <div class="stat-label">专注分钟</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon">🏆</div>
        <div class="stat-value">{{ records.filter(r => r.completed && r.state === 'Work').length }}</div>
        <div class="stat-label">总计番茄</div>
      </div>
    </div>

    <div class="chart-section">
      <div class="section-header">
        <h3>本周趋势</h3>
      </div>
      <div class="bar-chart">
        <div
          v-for="day in weeklyStats()"
          :key="day.date"
          class="bar-item"
        >
          <div class="bar-container">
            <div
              class="bar"
              :style="{ height: Math.max((day.count / getMaxCount()) * 80, 4) + 'px' }"
            >
              <span v-if="day.count > 0" class="bar-count">{{ day.count }}</span>
            </div>
          </div>
          <div class="bar-label">{{ day.date }}</div>
        </div>
      </div>
    </div>

    <div class="history-section">
      <div class="section-header">
        <h3>历史记录</h3>
        <span class="record-count">{{ filteredRecords().length }} 条</span>
      </div>
      <div class="history-list">
        <div
          v-for="(record, i) in filteredRecords().slice().reverse().slice(0, 15)"
          :key="i"
          class="history-item"
        >
          <div class="history-info">
            <div class="history-dot" />
            <div class="history-time">{{ record.start_time }}</div>
          </div>
          <div class="history-duration">{{ formatDuration(record.duration_secs) }}</div>
        </div>
        <div v-if="filteredRecords().length === 0" class="empty-state">
          <div class="empty-icon">📝</div>
          <div class="empty-text">暂无记录</div>
          <div class="empty-hint">开始第一个番茄钟吧</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stats-page {
  width: 100%;
  max-width: 480px;
  padding: 0 4px;
  height: 100%;
  overflow-y: auto;
}

.stats-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.stats-header h2 {
  font-family: 'Playfair Display', Georgia, serif;
  font-size: 24px;
  font-weight: 600;
  color: #F8F4EC;
}

.view-toggle {
  display: flex;
  gap: 4px;
  background: rgba(248, 244, 236, 0.06);
  padding: 4px;
  border-radius: 10px;
}

.toggle-btn {
  background: none;
  border: none;
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 500;
  color: rgba(248, 244, 236, 0.5);
  cursor: pointer;
  border-radius: 8px;
  transition: all 0.2s ease;
}

.toggle-btn:hover {
  color: rgba(248, 244, 236, 0.8);
}

.toggle-btn.active {
  background: #354A38;
  color: #F8F4EC;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  margin-bottom: 32px;
}

.stat-card {
  background: rgba(248, 244, 236, 0.04);
  border: 1px solid rgba(248, 244, 236, 0.06);
  border-radius: 16px;
  padding: 20px 16px;
  text-align: center;
  transition: all 0.2s ease;
}

.stat-card:hover {
  background: rgba(248, 244, 236, 0.06);
  transform: translateY(-2px);
}

.stat-icon {
  font-size: 24px;
  margin-bottom: 8px;
}

.stat-value {
  font-family: 'JetBrains Mono', monospace;
  font-size: 28px;
  font-weight: 500;
  color: #F39C12;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 12px;
  color: rgba(248, 244, 236, 0.5);
}

.chart-section {
  margin-bottom: 32px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h3 {
  font-size: 14px;
  font-weight: 600;
  color: rgba(248, 244, 236, 0.7);
}

.record-count {
  font-size: 12px;
  color: rgba(248, 244, 236, 0.4);
}

.bar-chart {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  height: 120px;
  padding: 0 8px;
  background: rgba(248, 244, 236, 0.02);
  border-radius: 12px;
  border: 1px solid rgba(248, 244, 236, 0.04);
}

.bar-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
}

.bar-container {
  height: 80px;
  display: flex;
  align-items: flex-end;
}

.bar {
  width: 28px;
  background: linear-gradient(180deg, #E74C3C 0%, #C0392B 100%);
  border-radius: 6px 6px 2px 2px;
  position: relative;
  transition: height 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  min-height: 4px;
}

.bar:hover {
  filter: brightness(1.1);
}

.bar-count {
  position: absolute;
  top: -22px;
  left: 50%;
  transform: translateX(-50%);
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  font-weight: 500;
  color: rgba(248, 244, 236, 0.7);
}

.bar-label {
  font-size: 11px;
  color: rgba(248, 244, 236, 0.4);
  margin-top: 8px;
}

.history-section {
  margin-bottom: 24px;
}

.history-list {
  background: rgba(248, 244, 236, 0.02);
  border-radius: 12px;
  border: 1px solid rgba(248, 244, 236, 0.04);
  overflow: hidden;
}

.history-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 16px;
  border-bottom: 1px solid rgba(248, 244, 236, 0.04);
  transition: background 0.2s ease;
}

.history-item:last-child {
  border-bottom: none;
}

.history-item:hover {
  background: rgba(248, 244, 236, 0.03);
}

.history-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.history-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #E74C3C;
}

.history-time {
  font-size: 13px;
  color: rgba(248, 244, 236, 0.7);
}

.history-duration {
  font-family: 'JetBrains Mono', monospace;
  font-size: 13px;
  font-weight: 500;
  color: #F39C12;
}

.empty-state {
  padding: 48px 24px;
  text-align: center;
}

.empty-icon {
  font-size: 32px;
  margin-bottom: 12px;
  opacity: 0.5;
}

.empty-text {
  font-size: 14px;
  color: rgba(248, 244, 236, 0.5);
  margin-bottom: 4px;
}

.empty-hint {
  font-size: 12px;
  color: rgba(248, 244, 236, 0.3);
}

@media (max-height: 500px) {
  .stats-header { margin-bottom: 12px; }
  .stats-header h2 { font-size: 18px; }
  .stats-cards { gap: 8px; margin-bottom: 16px; }
  .stat-card { padding: 12px 8px; }
  .stat-icon { font-size: 18px; margin-bottom: 4px; }
  .stat-value { font-size: 20px; }
  .chart-section { margin-bottom: 16px; }
  .bar-container { height: 50px; }
  .section-header { margin-bottom: 8px; }
  .empty-state { padding: 24px 16px; }
}
</style>