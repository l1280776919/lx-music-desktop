import SvgIcon from './SvgIcon.vue'

const svgModules = import.meta.glob('@renderer/assets/svgs/*.svg', { eager: true })
void svgModules

export default app => {
  app.component('svg-icon', SvgIcon)
}
