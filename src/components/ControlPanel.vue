<script setup lang="ts">
const props = defineProps<{
  state: string
  isPaused: boolean
  nextAction: string
  session: number
  sessionsBeforeLong: number
}>()

const emit = defineEmits<{
  start: []
  startBreak: [isLong: boolean]
  pause: []
  skip: []
  reset: []
}>()
</script>

<template>
  <div class="controls">
    <template v-if="state === 'Idle'">
      <button
        v-if="nextAction === 'Break'"
        class="btn btn-primary btn-break"
        @click="emit('startBreak', session > 0 && session % sessionsBeforeLong === 0)"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
          <path d="M8 5v14l11-7z" />
        </svg>
        <span>开始休息</span>
      </button>
      <button
        v-else
        class="btn btn-primary"
        @click="emit('start')"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
          <path d="M8 5v14l11-7z" />
        </svg>
        <span>开始专注</span>
      </button>
    </template>
    <template v-else>
      <button class="btn btn-ghost" @click="emit('reset')">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
          <path d="M3 3v5h5" />
        </svg>
      </button>
      <button class="btn btn-primary" @click="emit('pause')">
        <svg v-if="isPaused" width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
          <path d="M8 5v14l11-7z" />
        </svg>
        <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
          <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
        </svg>
        <span>{{ isPaused ? '继续' : '暂停' }}</span>
      </button>
      <button class="btn btn-ghost" @click="emit('skip')">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
          <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z" />
        </svg>
      </button>
    </template>
  </div>
</template>

<style scoped>
.controls {
  display: flex;
  gap: 16px;
  align-items: center;
  justify-content: center;
}

.btn {
  display: flex;
  align-items: center;
  gap: 8px;
  border: none;
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.btn-primary {
  background: linear-gradient(135deg, #E74C3C 0%, #C0392B 100%);
  color: white;
  padding: 12px 28px;
  border-radius: 32px;
  box-shadow: 0 4px 16px rgba(231, 76, 60, 0.3);
  font-size: 13px;
}

.btn-break {
  background: linear-gradient(135deg, #27AE60 0%, #1E8449 100%);
  box-shadow: 0 4px 16px rgba(39, 174, 96, 0.3);
}

.btn-break:hover {
  box-shadow: 0 6px 24px rgba(39, 174, 96, 0.4);
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 24px rgba(231, 76, 60, 0.4);
}

.btn-primary:active {
  transform: translateY(0);
  box-shadow: 0 2px 8px rgba(231, 76, 60, 0.3);
}

.btn-ghost {
  background: rgba(248, 244, 236, 0.06);
  color: rgba(248, 244, 236, 0.7);
  padding: 14px;
  border-radius: 50%;
}

.btn-ghost:hover {
  background: rgba(248, 244, 236, 0.1);
  color: rgba(248, 244, 236, 1);
}

.btn-ghost:active {
  background: rgba(248, 244, 236, 0.08);
}
</style>
