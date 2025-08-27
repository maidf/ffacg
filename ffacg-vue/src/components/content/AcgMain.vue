<template>
    <div v-if="animes">
        <anime-card v-for="(item, index) in animes" :key="index" :anime="item"></anime-card>
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

// 动漫加载
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