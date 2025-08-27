import { defineStore } from "pinia"

export const useDownStore = defineStore('down_store', () => {
    const down_count = ref(0)
    const down_start = ref<Record<string, number>>({})

    const inc_down = () => {
        down_count.value++
    }

    const clear_down = () => {
        down_count.value = 0
    }

    const set_start = (id: string) => {
        down_start.value[id] = Date.now()
    }

    const get_start = (id: string) => {
        return down_start.value[id]
    }

    return { down_count, down_start, inc_down, clear_down, set_start, get_start }
}, {
    persist: true,
})