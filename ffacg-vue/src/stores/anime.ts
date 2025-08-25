import axios from "axios"
import { defineStore } from "pinia"

export const useAnimeStore = defineStore("anime", () => {

    const get_animes = async (page: number) => {
        const res = await axios.get("https://api.animes.garden/resources", {
            params: { type: '动画', page: page, pageSize: 20 }
        })
        if (res.data.status === 'OK') {
            let arr = res.data.resources as Anime[]
            let uuid = 0;
            const r = arr.map((v) => ({ ...v, id: (uuid++).toString() }))
            return r as Anime[]
        }
    }

    return { get_animes }
})


export interface Anime {
    id: string
    title: string
    size: number
    magnet: string
    tracker: string
    fansub: Fansub
    publisher: Publisher
}

export interface Fansub {
    id: string
    name: string
    avatar: string
}

export interface Publisher {
    id: string
    name: string
    avatar: string
}

export interface AnimeHistory {
    id: string
    anime: Anime
    time: Date
}

export interface AnimeMask {
    id: string
    anime: Anime
    group: string
}