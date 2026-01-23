import { acceptHMRUpdate, defineStore } from 'pinia'

const versionString =
  import.meta.env.MODE === 'development' ? `${import.meta.env.VITE_APP_VERSION}-dev` : import.meta.env.VITE_APP_VERSION

export const useStore = defineStore('main', {
  state: () => ({
    debug: import.meta.env.MODE === 'development',
    version: versionString,
    isInitialized: false,
  }),

  actions: {
    async initApp() {
      this.isInitialized = true
      console.log('Project Launcher initialized!')
    },
  },

  getters: {
    isReady: (state) => {
      return state.isInitialized
    },
  },
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot))
}

// 导出其他 stores
export { useProjectStore } from './stores/project'
export { useLauncherStore } from './stores/launcher'
export { useSettingsStore } from './stores/settings'
