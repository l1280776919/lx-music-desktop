import "@common/error"
import { createApp } from 'vue'

import './tauri/globalData'
import { installAppLogger } from './tauri/logger'
import '@renderer/event'

import mountComponents from '@renderer/components'
import initPlugins from '@renderer/plugins'
import { i18nPlugin } from '@renderer/plugins/i18n'

import App from '@renderer/App.vue'
import router from '@renderer/router'

import { getSetting, getViewPrevState, updateSetting } from '@renderer/utils/ipc'
import { langList } from '@root/lang'
import type { I18n } from '@root/lang/i18n'

import { initSetting } from '@renderer/store/setting'
import { saveViewPrevState } from '@renderer/utils/data'
import { DEFAULT_SETTING } from '@common/constants'

installAppLogger()

router.afterEach((to) => {
  if (to.path != '/songList/detail') {
    saveViewPrevState({
      url: to.path,
      query: { ...to.query },
    })
  }
})

void getSetting().then(setting => {
  if (!setting['common.langId'] || !window.i18n.availableLocales.includes(setting['common.langId'])) {
    let langId: I18n['locale'] | null = null
    const locale = window.navigator.language.toLocaleLowerCase() as I18n['locale']
    if (window.i18n.availableLocales.includes(locale)) {
      langId = locale
    } else {
      for (const lang of langList) {
        if (lang.alternate == locale) {
          langId = lang.locale
          break
        }
      }
      langId ??= 'en-us'
    }
    setting['common.langId'] = langId
    void updateSetting({ 'common.langId': langId })
  }
  window.setLang(setting['common.langId'])
  window.i18n.setLanguage(setting['common.langId'])

  initSetting(setting)

  const currentRoute = router.currentRoute.value
  const isDefaultInitialRoute = currentRoute.path == DEFAULT_SETTING.viewPrevState.url &&
    Object.keys(currentRoute.query).length == 0

  const restoreRoutePromise = isDefaultInitialRoute
    ? getViewPrevState().then(state => {
        const targetPath = state?.url || DEFAULT_SETTING.viewPrevState.url
        const targetQuery = state?.query || {}
        const isSameRoute = targetPath == currentRoute.path &&
          JSON.stringify(targetQuery) == JSON.stringify(currentRoute.query)
        if (isSameRoute) return
        return router.replace({ path: targetPath, query: targetQuery })
      })
    : Promise.resolve()

  const app = createApp(App)
  app
    .use(router)
    .use(i18nPlugin)
  initPlugins(app)
  mountComponents(app)
  void restoreRoutePromise.finally(() => {
    app.mount('#root')
  })
})
