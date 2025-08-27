<template>
    <el-card class="mycard" shadow="hover">
        <template #header>{{ anime.title }}</template>
        <span v-if="anime.publisher">发布者:{{ anime.publisher.name }}</span> <br>
        <span v-if="anime.fansub">字幕组:{{ anime.fansub.name }}</span> <br>
        <span v-if="anime.size / 1024 > 1024">大小:{{ (anime.size / 1024 / 1024).toFixed(1) }}GB</span>
        <span v-else>大小:{{ (anime.size / 1024).toFixed(1) }}MB</span>
        <template #footer>
            <el-tooltip class="box-item" effect="dark" :content="anime_down.toString()" placement="top">
                <el-button :disabled="disabled" type="success" @click="handle_down">
                    {{ btn_text }}
                </el-button>
            </el-tooltip>
            <el-tooltip class="box-item" effect="dark" :content="anime_mask.toString()" placement="top">
                <el-button type="primary">收藏</el-button>
            </el-tooltip>
        </template>
    </el-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useDownStore } from '@/stores/downStore'
import type { Anime } from '@/stores/anime'

const { anime } = defineProps<{ anime: Anime }>()

const down_store = useDownStore()
const { set_start, get_start, inc_down } = down_store

const now = ref(Date.now())
setInterval(() => {
    now.value = Date.now()
}, 1000)


const remaining = computed(() => {
    const time = get_start(anime.id)
    if (!time) return 0
    const elapsed = Math.floor((now.value - time) / 1000)
    const left = 30 - elapsed
    return left > 0 ? left : 0
})

const disabled = computed(() => remaining.value > 0)
const btn_text = computed(() => (disabled.value ? `${remaining.value}s` : '下载'))

const handle_down = () => {
    if (disabled.value) return
    set_start(anime.id)
    if (anime.magnet) inc_down()
}

// 信息统计
const anime_down = ref(0)
const anime_mask = ref(0)
</script>

<style lang="scss" scoped>
.mycard {
    width: 300px;
    min-height: 300px;
    // margin-left: 12px;
    // margin-right: 12px;
    margin: 10px;
    box-sizing: 300px;
    border: 2px pink solid;
    overflow: hidden;

    display: inline-flex;
    flex-direction: column;

    :deep(.el-card__header) {
        background-color: pink;
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    :deep(.el-card__body) {
        flex-shrink: 0;
        padding: 10px;
        text-align: left;
        border-top: 1px solid #eee;
        background-color: #f9f9f9;
    }

    :deep(.el-card__footer) {
        flex-shrink: 0;
        padding: 10px;
        text-align: center;
        border-top: 1px solid #eee;
        // background-color: pink;
    }
}
</style>