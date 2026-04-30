const componentModules = import.meta.glob('./**/*.vue', { eager: true })
const upperFirst = str => str.charAt(0).toUpperCase() + str.slice(1)
const camelCase = str => str
  .replace(/(^|[/_-])(\w)/g, (_, __, c) => c.toUpperCase())
  .replace(/\//g, '')

const vueFileRxp = /\.vue$/

export default app => {
  Object.entries(componentModules).forEach(([fileName, componentConfig]) => {
    const filePath = fileName.replace(/^\.\//, '')

    if (!filePath.split('/').every((path, index, arr) => {
      const char = path.charAt(0)
      return vueFileRxp.test(path) || char.toUpperCase() !== char || arr[index + 1] == 'index.vue'
    })) return

    let componentName = upperFirst(camelCase(filePath.replace(/\.\w+$/, '')))

    if (componentName.endsWith('Index')) componentName = componentName.replace(/Index$/, '')

    app.component(componentName, componentConfig.default || componentConfig)
  })
}
