import { emit, listen } from '@tauri-apps/api/event'
import { event } from '@tauri-apps/api'

export type parameterConfig = {
    width: string,
    height: string
} 

export class Parameter {
    width: number = 0
    height: number = 0

    init(params: string) {
        const p = JSON.parse(params)
        this.width = Number(p.width)
        this.height = Number(p.height)
    }

    async listen() {
        await listen<string>('parameter-init', (event) => {
            console.log(event.payload)
            this.init(event.payload)
        })
    }
}