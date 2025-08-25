<template>
    <div v-if="animes">
        <el-card class="mycard" shadow="hover" v-for="item in animes" :key="item.id" :header="item.title">
            <span v-if="item.publisher">发布者:{{ item.publisher.name }}</span> <br>
            <span v-if="item.fansub">字幕组:{{ item.fansub.name }}</span> <br>
            <span v-if="item.size / 1024 > 1024">大小:{{ (item.size / 1024 / 1024).toFixed(1) }}GB</span>
            <span v-else>大小:{{ (item.size / 1024).toFixed(1) }}MB</span>
            <template #footer>
                <el-button type="success">下载</el-button>
                <el-button type="primary">播放</el-button>
            </template>
        </el-card>
        <div class="mypageblock">
            <el-pagination class="mypage" background layout="prev, pager, next, jumper" :page-count="100"
                v-model:current-page="current_page" @current-change="current_change" />
        </div>

    </div>
    <div v-else-if="loading">
        <el-skeleton :rows="5" animated />
    </div>
    <div v-else>
        <el-empty description="加载中..." />
    </div>
</template>

<script lang="ts" setup>
import { type Anime, useAnimeStore } from '@/stores/anime'
import { fa, tr } from 'element-plus/es/locales.mjs'
const animes = ref<Anime[] | null>(null)
const current_page = ref(1)

const loading = ref(false)

const current_change = async (val: number) => {
    loading.value = true
    current_page.value = val
    await get_animes(val).then((res) => {
        if (res)
            animes.value = res
    })
    loading.value = false
}

const anime_store = useAnimeStore()
const { get_animes } = anime_store
onMounted(async () => {
    loading.value = true
    await get_animes(current_page.value).then((res) => {
        if (res)
            animes.value = res
    })
    loading.value = false
})

watch(animes, () => {

})
</script>

<style scoped lang="scss">
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

    :deep(.el-card__body) {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    :deep(.el-card__footer) {
        flex-shrink: 0;
        padding: 10px;
        text-align: center;
        border-top: 1px solid #eee;
        background-color: #f9f9f9;
    }
}

.mypage {
    display: inline-flex;
}

.mypageblock {
    width: 600px;
    display: block;
    margin-left: auto;
    margin-right: auto;
}
</style>