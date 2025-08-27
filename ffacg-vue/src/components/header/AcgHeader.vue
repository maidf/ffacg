<template>
    <el-menu :router="true" :default-active="activeIndex" class="el-menu-demo" mode="horizontal" :ellipsis="false"
        @select="handleSelectMenu">
        <el-menu-item index="/home">
            <el-icon>
                <IconEpHouse />
            </el-icon>
            首页
        </el-menu-item>
        <el-menu-item index="search">
            <el-input v-model="search" placeholder="搜索" @select="handleSelectSearch" />
        </el-menu-item>
        <el-menu-item index="down" class="vertical-item" @click="handle_click_down">
            <el-badge :value="down_count" :max="99" :hidden="down_count === 0">
                <div class="vertical-content">
                    <el-icon>
                        <IconEpDownload />
                    </el-icon>
                    <span>下载</span>
                </div>
            </el-badge>
        </el-menu-item>
        <el-menu-item index="user" class="vertical-item">
            <el-icon>
                <IconEpUser />
            </el-icon>
            <span>用户</span>
        </el-menu-item>
        <el-menu-item disabled class="vertical-item">
        </el-menu-item>
    </el-menu>
</template>

<script lang="ts" setup>
import { useDownStore } from '@/stores/downStore'


// 下载状态
const down_store = useDownStore()
const down_count = computed(() => down_store.down_count)
const { clear_down } = down_store
const handle_click_down = () => {
    clear_down()
}

const activeIndex = ref('1')
const handleSelectMenu = (key: string, keyPath: string[]) => {
    console.log(key, keyPath)
}


const search = ref('')
const handleSelectSearch = (item: Record<string, any>) => {
    console.log(item)
}
</script>

<style lang="scss" scoped>
.el-menu--horizontal>.el-menu-item:nth-child(2) {
    margin-left: auto;
    margin-right: auto;
}

:deep(.vertical-item) {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 0 16px;

    .el-icon {
        margin: 0;
        font-size: 18px;
        margin-bottom: 2px;
    }

    span {
        font-size: 12px;
        line-height: 1;
    }
}

// 修复 badge 内部布局
.vertical-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    line-height: 1;
}
</style>