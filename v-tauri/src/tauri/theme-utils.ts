const RGB_Linear_Shade = (p: number, c0: string) => {
  const i = parseInt
  const r = Math.round
  let [a, b, c, d] = c0.split(',')
  const n = p < 0
  const t = n ? 0 : 255 * p
  const P = n ? 1 + p : 1 - p
  return 'rgb' + (d ? 'a(' : '(') + r(i(a[3] == 'a' ? a.slice(5) : a.slice(4)) * P + t) + ',' + r(i(b) * P + t) + ',' + r(i(c) * P + t) + (d ? ',' + d : ')')
}

const RGB_Alpha_Shade = (p: number, color: string) => {
  const i = parseInt
  const n = p < 0
  let [r, g, b, a] = color.split(',')
  r = r[3] == 'a' ? r.slice(5) : r.slice(4)
  let alpha = a ? parseFloat(a) : 1 - p
  if (a) {
    alpha = alpha - (n ? (1 - alpha) * p : alpha * p)
    alpha = n ? Math.max(0, alpha) : Math.min(1, alpha)
  } else {
    alpha = Math.min(1, alpha)
  }
  return `rgba(${i(r)}, ${i(g)}, ${i(b)}, ${alpha.toFixed(2)})`
}

const createFontDarkColors = (rgbaColor: string, isDarkFont?: boolean) => {
  const colors: Record<string, string> = {
    '--color-1000': rgbaColor,
  }
  const step = isDarkFont ? -0.015 : -0.05
  let preColor = rgbaColor
  for (let i = 1; i < 21; i += 1) {
    preColor = RGB_Linear_Shade(step, preColor)
    colors[`--color-${String(1000 - 50 * i).padStart(3, '0')}`] = preColor
  }
  return colors
}

const createFontColors = (rgbaColor: string | undefined, isDark?: boolean, isDarkFont?: boolean) => {
  rgbaColor ??= isDark ? 'rgb(229, 229, 229)' : 'rgb(33, 33, 33)'
  if (isDark) return createFontDarkColors(rgbaColor, isDarkFont)

  const colors: Record<string, string> = {
    '--color-1000': rgbaColor,
  }
  const step = ((isDarkFont ? 0.02 : 0.05) * (isDark ? -1 : 1))
  for (let i = 1; i < 21; i += 1) {
    colors[`--color-${String(1000 - 50 * i).padStart(3, '0')}`] = RGB_Linear_Shade(step * i, rgbaColor)
  }
  return colors
}

export const createThemeColors = (rgbaColor: string, fontRgbaColor?: string, isDark?: boolean, isDarkFont?: boolean) => {
  const colors: Record<string, string> = {
    '--color-primary': rgbaColor,
  }

  let preColor = rgbaColor
  for (let i = 1; i < 11; i += 1) {
    preColor = RGB_Linear_Shade(isDark ? 0.2 : -0.1, preColor)
    colors[`--color-primary-dark-${i * 100}`] = preColor
    for (let j = 1; j < 10; j += 1) {
      colors[`--color-primary-dark-${i * 100}-alpha-${j * 100}`] = RGB_Alpha_Shade(0.1 * j, preColor)
      colors[`--color-primary-alpha-${j * 100}`] = RGB_Alpha_Shade(0.1 * j, rgbaColor)
    }
  }
  preColor = rgbaColor
  for (let i = 1; i < 10; i += 1) {
    preColor = RGB_Linear_Shade(isDark ? -0.1 : 0.2, preColor)
    colors[`--color-primary-light-${i * 100}`] = preColor
    for (let j = 1; j < 10; j += 1) {
      colors[`--color-primary-light-${i * 100}-alpha-${j * 100}`] = RGB_Alpha_Shade(0.1 * j, preColor)
    }
  }
  preColor = RGB_Linear_Shade(isDark ? -0.35 : 1, preColor)
  colors['--color-primary-light-1000'] = preColor
  for (let j = 1; j < 10; j += 1) {
    colors[`--color-primary-light-1000-alpha-${j * 100}`] = RGB_Alpha_Shade(0.1 * j, preColor)
  }
  colors['--color-theme'] = isDark ? colors['--color-primary-light-900'] : rgbaColor

  return { ...colors, ...createFontColors(fontRgbaColor, isDark, isDarkFont) }
}
