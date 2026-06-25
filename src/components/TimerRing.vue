<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const props = defineProps<{
  progress: number
  state: string
}>()

const containerRef = ref<HTMLElement>()
const size = ref(280)

const updateSize = () => {
  if (!containerRef.value) return
  const parent = containerRef.value.parentElement
  if (!parent) return
  const w = parent.clientWidth
  const h = parent.clientHeight
  size.value = Math.min(w - 48, h - 48, 320)
  size.value = Math.max(size.value, 160)
}

onMounted(() => {
  updateSize()
  window.addEventListener('resize', updateSize)
})

onUnmounted(() => {
  window.removeEventListener('resize', updateSize)
})

const stroke = 6
const radius = () => (size.value - stroke) / 2
const circumference = () => 2 * Math.PI * radius()
const offset = () => circumference() * (1 - props.progress)

const getStrokeColor = () => {
  switch (props.state) {
    case 'Work':
      return 'url(#workGradient)'
    case 'ShortBreak':
    case 'LongBreak':
      return 'url(#breakGradient)'
    default:
      return 'url(#idleGradient)'
  }
}
</script>

<template>
  <div ref="containerRef" class="timer-ring" :style="{ width: size + 'px', height: size + 'px' }">
    <svg :width="size" :height="size" class="ring-svg">
      <defs>
        <linearGradient id="workGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#E74C3C" />
          <stop offset="100%" stop-color="#F39C12" />
        </linearGradient>
        <linearGradient id="breakGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#27AE60" />
          <stop offset="100%" stop-color="#2ECC71" />
        </linearGradient>
        <linearGradient id="idleGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="rgba(248, 244, 236, 0.2)" />
          <stop offset="100%" stop-color="rgba(248, 244, 236, 0.1)" />
        </linearGradient>
        <filter id="glow">
          <feGaussianBlur stdDeviation="3" result="coloredBlur" />
          <feMerge>
            <feMergeNode in="coloredBlur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>

      <circle
        class="ring-bg"
        :cx="size / 2"
        :cy="size / 2"
        :r="radius()"
        :stroke-width="stroke"
        fill="none"
      />

      <circle
        class="ring-progress"
        :cx="size / 2"
        :cy="size / 2"
        :r="radius()"
        :stroke-width="stroke"
        fill="none"
        :stroke-dasharray="circumference()"
        :stroke-dashoffset="offset()"
        :stroke="getStrokeColor()"
        filter="url(#glow)"
      />

      <g class="leaf-decoration" :transform="`translate(${size / 2}, ${stroke / 2 + 4})`">
        <path
          d="M-8,0 Q0,-12 8,0 Q0,4 -8,0"
          :fill="state === 'Work' ? '#27AE60' : state === 'Idle' ? 'rgba(248, 244, 236, 0.2)' : '#2ECC71'"
          class="leaf"
        />
        <path
          d="M-4,0 Q0,-8 4,0 Q0,2 -4,0"
          :fill="state === 'Work' ? '#1E8449' : state === 'Idle' ? 'rgba(248, 244, 236, 0.15)' : '#27AE60'"
          class="leaf-inner"
        />
      </g>
    </svg>

    <div class="ring-content">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.timer-ring {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.ring-svg {
  transform: rotate(-90deg);
  filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.2));
}

.ring-bg {
  stroke: var(--surface);
}

.ring-progress {
  stroke-linecap: round;
  transition: stroke-dashoffset 0.8s cubic-bezier(0.4, 0, 0.2, 1);
}

.leaf-decoration {
  transform-origin: center;
}

.leaf {
  opacity: 0.9;
}

.leaf-inner {
  opacity: 0.7;
}

.ring-content {
  position: absolute;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
</style>
