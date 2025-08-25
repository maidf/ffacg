<template>
    <el-menu :router="true" :default-active="activeIndex" class="el-menu-demo" mode="horizontal" :ellipsis="false"
        @select="handleSelectMenu">
        <el-menu-item index="/home">
            <el-icon>
                <IconEpHouse />
            </el-icon>
            首页
        </el-menu-item>
        <el-menu-item index="2">
            <el-input v-model="search" placeholder="搜索" @select="handleSelectSearch" />
        </el-menu-item>
        <el-menu-item index="3">
            <el-icon>
                <IconEpUser />
            </el-icon>
            用户
        </el-menu-item>
    </el-menu>
</template>

<script lang="ts" setup>

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
</style>