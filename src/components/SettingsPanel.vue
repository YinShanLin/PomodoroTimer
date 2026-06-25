<script setup lang="ts">
import { ref, watch } from 'vue'
import type { TimerSettings } from '../composables/useTimer'

const props = defineProps<{
  settings: TimerSettings
}>()

const emit = defineEmits<{
  save: [settings: TimerSettings]
  close: []
}>()

const toMinutes = (secs: number) => Math.floor(secs / 60)
const toSecs = (mins: number) => mins * 60

const local = ref({
  work_duration: toMinutes(props.settings.work_duration),
  short_break: toMinutes(props.settings.short_break),
  long_break: toMinutes(props.settings.long_break),
  sessions_before_long: props.settings.sessions_before_long,
  sound_enabled: props.settings.sound_enabled,
})

watch(() => props.settings, (val) => {
  local.value = {
    work_duration: toMinutes(val.work_duration),
    short_break: toMinutes(val.short_break),
    long_break: toMinutes(val.long_break),
    sessions_before_long: val.sessions_before_long,
    sound_enabled: val.sound_enabled,
  }
})

const handleSave = () => {
  emit('save', {
    work_duration: toSecs(local.value.work_duration),
    short_break: toSecs(local.value.short_break),
    long_break: toSecs(local.value.long_break),
    sessions_before_long: local.value.sessions_before_long,
    sound_enabled: local.value.sound_enabled,
  })
}
</script>

<template>
  <div class="settings-overlay" @click.self="emit('close')">
    <div class="settings-panel">
      <div class="panel-header">
        <h2>设置</h2>
        <button class="close-btn" @click="emit('close')">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="settings-content">
        <div class="setting-group">
          <div class="setting-item">
            <label>专注时长</label>
            <div class="input-wrapper">
              <input
                type="number"
                v-model.number="local.work_duration"
                :min="1"
                :max="120"
              />
              <span class="input-suffix">分钟</span>
            </div>
          </div>

          <div class="setting-item">
            <label>短休息时长</label>
            <div class="input-wrapper">
              <input
                type="number"
                v-model.number="local.short_break"
                :min="1"
                :max="30"
              />
              <span class="input-suffix">分钟</span>
            </div>
          </div>

          <div class="setting-item">
            <label>长休息时长</label>
            <div class="input-wrapper">
              <input
                type="number"
                v-model.number="local.long_break"
                :min="1"
                :max="60"
              />
              <span class="input-suffix">分钟</span>
            </div>
          </div>

          <div class="setting-item">
            <label>长休息间隔</label>
            <div class="input-wrapper">
              <input
                type="number"
                v-model.number="local.sessions_before_long"
                :min="2"
                :max="10"
              />
              <span class="input-suffix">轮</span>
            </div>
          </div>
        </div>

        <div class="setting-group">
          <div class="setting-item toggle-item">
            <label>提示音</label>
            <button
              class="toggle"
              :class="{ active: local.sound_enabled }"
              @click="local.sound_enabled = !local.sound_enabled"
            >
              <div class="toggle-thumb" />
            </button>
          </div>
        </div>
      </div>

      <div class="setting-actions">
        <button class="btn btn-secondary" @click="emit('close')">取消</button>
        <button class="btn btn-primary" @click="handleSave">保存设置</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.settings-panel {
  background: var(--bg-secondary);
  border-radius: 20px;
  padding: 0;
  width: 380px;
  max-width: calc(100vw - 32px);
  max-height: calc(100vh - 48px);
  overflow-y: auto;
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
  border: 1px solid rgba(248, 244, 236, 0.08);
  animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px 24px 0;
}

.panel-header h2 {
  font-family: var(--font-display);
  font-size: 22px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.close-btn {
  background: none;
  border: none;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 8px;
  border-radius: 8px;
  transition: all 0.2s ease;
}

.close-btn:hover {
  color: var(--text-secondary);
  background: var(--surface);
}

.settings-content {
  padding: 24px;
}

.setting-group {
  margin-bottom: 24px;
}

.setting-group:last-child {
  margin-bottom: 0;
}

.setting-item {
  margin-bottom: 20px;
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-item label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-muted);
  margin-bottom: 8px;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.input-wrapper input {
  width: 100%;
  padding: 12px 48px 12px 16px;
  border: 1px solid rgba(248, 244, 236, 0.1);
  border-radius: 12px;
  background: rgba(248, 244, 236, 0.04);
  color: var(--text-primary);
  font-family: var(--font-mono);
  font-size: 15px;
  font-weight: 400;
  box-sizing: border-box;
  transition: all 0.2s ease;
}

.input-wrapper input:focus {
  outline: none;
  border-color: var(--accent-red);
  background: rgba(231, 76, 60, 0.04);
  box-shadow: 0 0 0 3px rgba(231, 76, 60, 0.1);
}

.input-suffix {
  position: absolute;
  right: 16px;
  font-size: 13px;
  color: var(--text-tertiary);
}

.toggle-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.toggle-item label {
  margin-bottom: 0;
}

.toggle {
  position: relative;
  width: 48px;
  height: 28px;
  background: rgba(248, 244, 236, 0.1);
  border: none;
  border-radius: 14px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.toggle.active {
  background: var(--accent-red);
}

.toggle-thumb {
  position: absolute;
  top: 4px;
  left: 4px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: 50%;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle.active .toggle-thumb {
  transform: translateX(20px);
}

.setting-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  padding: 16px 24px 24px;
  border-top: 1px solid rgba(248, 244, 236, 0.06);
}

.btn {
  padding: 12px 24px;
  border: none;
  border-radius: 12px;
  font-family: var(--font-body);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary {
  background: linear-gradient(135deg, var(--accent-red) 0%, var(--accent-red-dark) 100%);
  color: white;
  box-shadow: 0 2px 8px rgba(231, 76, 60, 0.3);
}

.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(231, 76, 60, 0.4);
}

.btn-secondary {
  background: var(--surface);
  color: var(--text-secondary);
}

.btn-secondary:hover {
  background: var(--surface-active);
  color: var(--text-primary);
}
</style>