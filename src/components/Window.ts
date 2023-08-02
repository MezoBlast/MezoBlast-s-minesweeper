import { WebviewWindow, appWindow, getAll, WindowOptions } from '@tauri-apps/api/window'
import { relaunch, exit } from '@tauri-apps/api/process'
import { emit, listen } from '@tauri-apps/api/event'

export type winConfig = {
    label: string,
    others: WindowOptions
}

export class Windows {
    mainWin!: WebviewWindow;

    getWin(label: string) {
        return WebviewWindow.getByLabel(label)
    }

    getAllWin() {
        return getAll()
    }

    async createWin(options: winConfig) {
        let label = options.label
        let args: WindowOptions = options.others
        console.log(args)

        const existWin = getAll().find(w => w.label == label)
        if(existWin) {
            if(existWin.label.indexOf('main') == -1) {
                await existWin?.unminimize()
                await existWin?.setFocus()
                return
            }
            await existWin?.close()
        }

        let win = new WebviewWindow(label, args)
        
        if(args.maximized && args.resizable) {
            win.maximize()
        }

        win.once('tauri://created', async() => {
            console.log('window create success!')
        })

        win.once('tauri://error', async() => {
            console.log('window create error!')
        })
    }

    async listen() {
        await listen<winConfig>('tauri-win-create', (event) => {
            console.log(event)
            this.createWin(event.payload)
        })

        await listen('tauri-win-show', async(event) => {
            if(appWindow.label.indexOf('main') == -1) return
            await appWindow.show()
            await appWindow.unminimize()
            await appWindow.setFocus()
        })

        await listen('tauri-win-hide', async(event) => {
            if(appWindow.label.indexOf('main') == -1) return
            await appWindow.hide()
        })

        await listen('tauri-win-exit', async(event) => {
            await exit()
        })

        await listen('tauri-win-relaunch', async(event) => {
            await relaunch()
        })

        await listen<winConfig>('tauri-win-setdata', async(event) => {
            await emit('win-postdata', event.payload)
        })
    }
}
