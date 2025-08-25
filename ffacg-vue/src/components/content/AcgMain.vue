<template>
    <div v-if="animes">
        <el-card class="mycard" shadow="hover" v-for="item in animes" :key="item.id" :header="item.title">
            <p v-if="item.publisher">发布者:{{ item.publisher.name }}</p>
            <p v-if="item.fansub">字幕组:{{ item.fansub.name }}</p>
            <p v-if="item.size / 1024 > 1024">大小:{{ (item.size / 1024 / 1024).toFixed(1) }}GB</p>
            <p v-else>大小:{{ (item.size / 1024).toFixed(1) }}MB</p>
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
    height: 300px;
    // margin-left: 12px;
    // margin-right: 12px;
    margin: 12px;
    display: inline-block;
    box-sizing: 330px;
    border: 2px pink solid;
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